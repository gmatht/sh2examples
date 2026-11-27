#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
use feature 'say';

our $CHILD_ERROR = 0;
my $__argc = @__sh_args;
my $desc;
my $arg;
my $stdout;
my $rc;
my $stderr;
my $rc_zero;
my @cmd;

# __SH_RUN_EXT_BEGIN (UU-FFI.md external-command seam)
# uu-ffi .so present: in-process coreutils via uu_run (capture = temp-file
# fd-1 redirect around the in-process call — no pipe, so no concurrent-
# drain deadlock, and no C-side wrapper .so needed). Absent: guarded
# fork/exec fallback (one-time warning). The bash -c below is
# feature-guarded — check_qx strips this block before scanning.
my $__SH_UU = 0;
my $__SH_UU_FFI;
our $__sh_uu_rc = 0;
my %__SH_UU_OK = map { $_ => 1 } qw(cat wc ls sort sed awk grep date sha256sum uname basename dirname cp);
BEGIN {
    my @cand;
    push @cand, $ENV{SH2_UU_LIB} if defined $ENV{SH2_UU_LIB};
    for my $up ('', '../', '../../', '../../../', '../../../../', '../../../../../') {
        push @cand, "${up}runtime/lib/libcoreutils_ffi.so";
    }
    push @cand, 'libcoreutils_ffi.so';
    for my $lib (@cand) {
        next unless defined $lib && -e $lib;
        my $ok = eval {
            require FFI::Platypus;
            my $ffi = FFI::Platypus->new(lib => [$lib]);
            $ffi->attach([uu_run => '__sh_uu_run_ffi'] => ['string', 'int', 'string[]'] => 'int');
            $__SH_UU_FFI = $ffi;
            $__SH_UU = 1;
            1;
        };
        last if $ok;
    }
    unless ($__SH_UU) {
        warn "sh2perl: uu-ffi .so (libcoreutils_ffi) not found — external commands fall back to fork/exec (slower)\n";
    }
}
my $__SH_UU_SEQ = 0;
sub __sh_uu_run {
    my @argv = @_;
    if ($__SH_UU && ($__SH_UU_OK{ $argv[0] // '' } // 0)) {
        $__sh_uu_rc = __sh_uu_run_ffi($argv[0], scalar(@argv), \@argv);
        return;
    }
    system(@argv);
    $__sh_uu_rc = $? >> 8;
}
# ── pipeline front end (coreutils-ffi-pipeline C ABI) ──────────────
# uu_pipeline_run_str runs a LINEAR pipeline (a|b|c) with redirections
# in-process, so the Perl backend never emits `bash -c` for those shapes.
my $__SH_PIPE = 0;
my $__SH_PIPE_TRIED = 0;
# Shell single-quote a runtime value so the pipeline parser sees one argv.
sub __sh_q {
    my $s = shift;
    $s = '' unless defined $s;
    $s =~ s/'/'\\''/g;
    return "'$s'";
}
sub __sh_pipe_load {
    return 1 if $__SH_PIPE;
    return 0 if $__SH_PIPE_TRIED;
    $__SH_PIPE_TRIED = 1;
    my @cand;
    push @cand, $ENV{SH2_UU_PIPELINE_LIB} if defined $ENV{SH2_UU_PIPELINE_LIB};
    for my $up ('', '../', '../../', '../../../', '../../../../', '../../../../../') {
        push @cand, "${up}runtime/lib/libcoreutils_ffi_pipeline.so";
    }
    push @cand, 'libcoreutils_ffi_pipeline.so';
    for my $lib (@cand) {
        next unless defined $lib && -e $lib;
        my $ok = eval {
            require FFI::Platypus;
            my $ffi = FFI::Platypus->new(api => 2);
            $ffi->lib($lib);
            $ffi->attach([uu_pipeline_run_str => '__sh_pipe_run_str'] => ['string','string'] => 'int');
            $ffi->attach([uu_pipeline_last_code => '__sh_pipe_last_code'] => [] => 'int');
            $__SH_PIPE = 1;
            1;
        };
        last if $ok;
    }
    # Point the process backends at the multicall helper shipped alongside.
    if ($__SH_PIPE && !defined $ENV{UU_FFI_MULTICALL}) {
        for my $up ('', '../', '../../', '../../../') {
            my $m = "${up}runtime/lib/coreutils-ffi-multicall";
            if (-x $m) { $ENV{UU_FFI_MULTICALL} = $m; last; }
        }
    }
    $__SH_PIPE;
}
sub __sh_pipe_unsupported_util {
    my ($cmd) = @_;
    # The reconstructed command quotes each stage's util name. These are not
    # in the pipeline .so (non-uutils tools / shell builtins); routing them
    # in-process is a race between rc 127 (unknown util) and rc 141 (SIGPIPE
    # from the neighbour writing into the dead stage) — 141 is a real exit
    # code, so the caller skipped the shell fallback and printed nothing
    # (parse-sq-with-for, intermittently). Send them to the shell up front.
    return $cmd =~ m{(?:^|\|)\s*'(?:perl|python3?|bc|find|strings|gunzip|gzip|zstd|bzip2|xz|tar|unzip|mapfile|readarray|ruby|node|php)'};
}
sub __sh_pipe_run {
    my ($cmd, $opts) = @_;
    # `$$` in rebuilt shell text is the SCRIPT's pid in bash (one shell for
    # the whole program). Each rebuilt command here runs in a FRESH `bash -c`,
    # whose own `$$` would differ per call — so a source `2>/tmp/e.$$` writer
    # and a later `cat /tmp/e.$$` reader (single-quoted by __sh_q, never
    # expanded) could never agree (realpath-cmdsub's stderr capture). Expand
    # to the Perl pid up front so writer, reader and `unlink("…" . ($$))`
    # all name the same file.
    $cmd =~ s/\$\$/$$/g;
    if (__sh_pipe_load() && !__sh_pipe_unsupported_util($cmd)) {
        my $rc = __sh_pipe_run_str($cmd, defined $opts ? $opts : '');
        # `-2` parse / `-3` backend; `127` is the multicall's "unknown util"
        # (not a uutils command) — run those through the shell instead.
        if ($rc >= 0 && $rc != 127) { $CHILD_ERROR = $rc; return $rc; }
    }
    # Not parseable / unknown util / no backend / no .so: run it via the shell.
    # argv0 after the command STRING ($0, not `bash`): a rebuilt command may
    # contain `$0`/`${0}` from the source (`script_dir=$(cd -- "$(dirname
    # "$0")" && pwd)`), and bash passes the script's $0 down to every subshell
    # and pipeline. With argv0 defaulting to "bash", `dirname ${0}` saw "bash"
    # → "." → pwd returned the CWD instead of the script's directory
    # (qx-var-builtin-cd).
    system('bash', '-c', $cmd, $0);
    $CHILD_ERROR = $? >> 8;
    return $CHILD_ERROR;
}
sub __sh_pipe_capture {
    my ($cmd, $opts) = @_;
    # See __sh_pipe_run: one Perl pid for writer and reader of `…$$`.
    $cmd =~ s/\$\$/$$/g;
    if (__sh_pipe_load() && !__sh_pipe_unsupported_util($cmd)) {
        my $tmp = "/tmp/sh2_pipe_${$}_" . ($__SH_UU_SEQ++) . ".cap";
        my $rc = __sh_pipe_run_str($cmd . ' > ' . $tmp, defined $opts ? $opts : '');
        if ($rc >= 0 && $rc != 127) {
            my $out = '';
            if (open(my $rf, '<', $tmp)) { $out = do { local $/; <$rf> } // ''; close $rf; }
            unlink $tmp;
            $CHILD_ERROR = $rc;
            $out =~ s/\n+\z//;
            return $out;
        }
        unlink $tmp;
    }
    my $out = do {
        open(my $fh, '-|', 'bash', '-c', $cmd, $0) or do { $CHILD_ERROR = 127; return ''; };
        my $r = do { local $/; <$fh> };
        close $fh;
        $r;
    };
    $CHILD_ERROR = $? >> 8;
    $out =~ s/\n+\z//;
    return $out // '';
}
# __SH_RUN_EXT_END

$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 9
sub try {
    local @__sh_args = @_;
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    my $desc = $__sh_args[0];
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    shift @__sh_args;
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    my @cmd = (@__sh_args);
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    print "==== " . ($desc) . " ====\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    print("  \$ ");
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    my $__sh_loop_rc0 = 0;
    for my $__arg ((@cmd)) {
        $arg = $__arg;
        $CHILD_ERROR = (($arg =~ '[[:space:]]')) ? 0 : 1;
        if (($arg =~ '[[:space:]]')) {
            print "'$arg' ";
            $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        } else {
            print "$arg ";
            $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        }
        $__sh_loop_rc0 = $? >> 8;
    }
$? = ($__sh_loop_rc0) << 8; $CHILD_ERROR = $? >> 8;
    print "\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    my $stdout;
    my $stderr;
    my $rc;
    $stdout = do { no strict 'vars'; no warnings; local $ENV{cmd} = join(' ', @cmd); __sh_pipe_capture(join(' ', @cmd) . ' 2> "/tmp/realpath_stderr.$$"', "pipefail") };
    $rc = $CHILD_ERROR;
    $stderr = do { my $__c = __sh_pipe_capture(q{'cat' } . __sh_q('/tmp/realpath_stderr.$$') . ' 2>/dev/null'); $CHILD_ERROR = 0; $__c };
    do { unlink("/tmp/realpath_stderr." . ($$)); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
    if ($stdout ne "") {
        $CHILD_ERROR = (($stdout =~ /^(?s).*.*$/)) ? 0 : 1;
        if (($stdout =~ /^(?s).*.*$/)) {
            print("  stdout (NUL‑terminated): ");
            $? = 0 << 8; $CHILD_ERROR = $? >> 8;
            {
                my $__sh2_pcap = '';
                { local *STDOUT; open(STDOUT, '>', \$__sh2_pcap) or die "capture: $!\n";
                    print($stdout);
                    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
                ; }
                my $__sh2_ptail = q{'od' '-A' 'n' '-t' 'x1z'};
                open(my $__sh2_pfh, '|-', 'bash', '-c', $__sh2_ptail, $0) or die "sh2perl: pipe open failed: $!\n";
                print $__sh2_pfh $__sh2_pcap;
                close $__sh2_pfh;
                $? = ($? >> 8) << 8; $CHILD_ERROR = $? >> 8;
            }
            print("\n");
            $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        } else {
            print "  stdout: $stdout\n";
            $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        }
    } else {
        print("  stdout: (empty)\n");
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    }
    if ($stderr ne "") {
        print "  stderr: $stderr\n";
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    }
    my $__sh2_pf_bad0 = 0;
    printf("  exit code: %d\n\n", (do {
    my ($__pf_s, $__pf_v, $__pf_bad, $__pf_msg) = ($rc, 0, 0, 'invalid number');
    my $__pf_t = $__pf_s;
    $__pf_t =~ s/\A[ \t\n]+//;
    if ($__pf_t =~ /\A([+-]?)0[xX]([0-9a-fA-F]*)(.*)\z/s) {
        $__pf_v = length($__sh_args[1]) ? hex($__sh_args[1]) : 0;
        $__pf_v = -$__pf_v if $__sh_args[0] eq '-';
        if (length($__sh_args[2]) || !length($__sh_args[1])) { $__pf_bad = 1; $__pf_msg = 'invalid hex number'; }
    } elsif ($__pf_t =~ /\A([+-]?)0[bB]([01]*)(.*)\z/s) {
        $__pf_v = length($__sh_args[1]) ? oct('0b' . $__sh_args[1]) : 0;
        $__pf_v = -$__pf_v if $__sh_args[0] eq '-';
        $__pf_bad = 1 if length($__sh_args[2]) || !length($__sh_args[1]);
    } elsif ($__pf_t =~ /\A([+-]?)0([0-7]*)(.*)\z/s) {
        $__pf_v = length($__sh_args[1]) ? oct('0' . $__sh_args[1]) : 0;
        $__pf_v = -$__pf_v if $__sh_args[0] eq '-';
        if (length($__sh_args[2])) { $__pf_bad = 1; $__pf_msg = 'invalid octal number'; }
    } elsif ($__pf_t =~ /\A([+-]?)([0-9]+)(.*)\z/s) {
        $__pf_v = 0 + $__sh_args[1];
        $__pf_v = -$__pf_v if $__sh_args[0] eq '-';
        $__pf_bad = 1 if length($__sh_args[2]);
    } else {
        $__pf_bad = 1;
    }
    if ($__pf_bad) { print STDERR "printf: $__pf_s: $__pf_msg\n"; $__sh2_pf_bad0 = 1; }
    $__pf_v
}));
    $? = (($__sh2_pf_bad0 ? 1 : 0)) << 8; $CHILD_ERROR = $? >> 8;
}
try('realpath (default, --physical) on a simple file', "realpath", "/bin");  # line 64
try('realpath on a two‑hop symlink chain: /usr/bin/vi → /etc/alternatives/vi → /usr/bin/vim.basic', "realpath", "/usr/bin/vi");  # line 67
try('realpath on a relative symlink: /usr/local/bin/pi', "realpath", "/usr/local/bin/pi");  # line 70
try('realpath on a regular file (no symlinks)', "realpath", "/etc/hostname");  # line 73
try('realpath on a directory with .. component', "realpath", "/tmp/..");  # line 76
try('--canonicalize-existing on an existing path', "realpath", "--canonicalize-existing", "/usr/bin/sh");  # line 85
try('--canonicalize-existing on a path with a missing last component (should fail)', "realpath", "--canonicalize-existing", "/tmp/no_such_file_xyzzy"); if ($CHILD_ERROR != 0) {  # line 88
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
try('--canonicalize-missing on a path with a non‑existent leaf', "realpath", "--canonicalize-missing", "/tmp/no_such_file_xyzzy");  # line 96
try('--canonicalize-missing on a completely imaginary path', "realpath", "--canonicalize-missing", "/nonexistent/deeply/missing/file");  # line 99
try('--canonicalize-missing on a path with .. and non‑existent parts', "realpath", "--canonicalize-missing", "/tmp/../nonexistent/../foo");  # line 102
try('--logical: /bin/..  (bin is a symlink, logical resolves .. before following it)', "realpath", "--logical", "/bin/..");  # line 110
try('default (--physical) for comparison: /bin/..', "realpath", "--physical", "/bin/..");  # line 113
try('--physical: /bin/..  (explicit, same as default)', "realpath", "--physical", "/bin/..");  # line 121
try('--strip (no symlink expansion) on /usr/bin/vi', "realpath", "--strip", "/usr/bin/vi");  # line 129
try('--strip on /bin  (symlink /bin → usr/bin)', "realpath", "--strip", "/bin");  # line 132
try('compare: default (--physical) on /bin', "realpath", "/bin");  # line 135
try('--relative-to=/usr/bin for /usr/bin/sh', "realpath", "--relative-to=/usr/bin", "/usr/bin/sh");  # line 143
try('--relative-to=/tmp for /etc/hostname', "realpath", "--relative-to=/tmp", "/etc/hostname");  # line 146
try('--relative-to=/ for /etc/hostname', "realpath", "--relative-to=/", "/etc/hostname");  # line 149
try('--relative-base=/etc for /etc/hostname (below /etc → relative)', "realpath", "--relative-base=/etc", "/etc/hostname");  # line 157
try('--relative-base=/etc for /usr/bin/sh (not below /etc → absolute)', "realpath", "--relative-base=/etc", "/usr/bin/sh");  # line 160
try('--relative-base=/ with --relative-to=/tmp  (combined)', "realpath", "--relative-base=/", "--relative-to=/tmp", "/etc/hostname", "/usr/bin/sh");  # line 163
print "==== --zero with two paths ====\n";  # line 173
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print("  \$ realpath --zero /bin /usr/bin/sh\n");  # line 174
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
__sh_uu_run('realpath', '--zero', '/bin', '/usr/bin/sh');$? = ($__sh_uu_rc) << 8; $CHILD_ERROR = $? >> 8;  # line 175
$rc_zero = $CHILD_ERROR;  # line 175
print("  (raw output with NULs above; use od to verify):\n");  # line 177
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print("  ");  # line 178
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = (__sh_pipe_run(q{'realpath' '--zero' '/bin' '/usr/bin/sh' | 'od' '-A' 'n' '-t' 'x1z' | 'head' '-3'}, "pipefail")) << 8; $CHILD_ERROR = $? >> 8;  # line 178
my $__sh2_pf_bad1 = 0;  # line 179
printf("  exit code: %d\n\n", (do {
    my ($__pf_s, $__pf_v, $__pf_bad, $__pf_msg) = ($rc_zero, 0, 0, 'invalid number');
    my $__pf_t = $__pf_s;
    $__pf_t =~ s/\A[ \t\n]+//;
    if ($__pf_t =~ /\A([+-]?)0[xX]([0-9a-fA-F]*)(.*)\z/s) {
        $__pf_v = length($2) ? hex($2) : 0;
        $__pf_v = -$__pf_v if $1 eq '-';
        if (length($3) || !length($2)) { $__pf_bad = 1; $__pf_msg = 'invalid hex number'; }
    } elsif ($__pf_t =~ /\A([+-]?)0[bB]([01]*)(.*)\z/s) {
        $__pf_v = length($2) ? oct('0b' . $2) : 0;
        $__pf_v = -$__pf_v if $1 eq '-';
        $__pf_bad = 1 if length($3) || !length($2);
    } elsif ($__pf_t =~ /\A([+-]?)0([0-7]*)(.*)\z/s) {
        $__pf_v = length($2) ? oct('0' . $2) : 0;
        $__pf_v = -$__pf_v if $1 eq '-';
        if (length($3)) { $__pf_bad = 1; $__pf_msg = 'invalid octal number'; }
    } elsif ($__pf_t =~ /\A([+-]?)([0-9]+)(.*)\z/s) {
        $__pf_v = 0 + $2;
        $__pf_v = -$__pf_v if $1 eq '-';
        $__pf_bad = 1 if length($3);
    } else {
        $__pf_bad = 1;
    }
    if ($__pf_bad) { print STDERR "printf: $__pf_s: $__pf_msg\n"; $__sh2_pf_bad1 = 1; }
    $__pf_v
}));
$? = (($__sh2_pf_bad1 ? 1 : 0)) << 8; $CHILD_ERROR = $? >> 8;
try('--quiet suppresses error message for a truly invalid path', "realpath", "--quiet", "/nonexistent_dir_xyzzy/foo"); if ($CHILD_ERROR != 0) {  # line 188
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
try('without --quiet for comparison (stderr appears)', "realpath", "/nonexistent_dir_xyzzy/foo"); if ($CHILD_ERROR != 0) {  # line 191
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
print "=== All tests completed ===\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
