#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $main_exit_code = 0;
our $CHILD_ERROR = 0;
our $__sh2_last_bg = '';
my $DEFAULT_ADAPTER;
my $ADAPTER;
my $ADAPTER3;
my $DOINGS;
my $ERRORS;
my $N;
my $BGPID;
my $CPID;

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

chdir("/tmp") or die "cd: $!\n";  # line 32
open my $__fh, '>', "grep_p_routes.txt" or die "Cannot write file: $!\n";  # line 35
print {$__fh} "default via 10.0.0.1 dev eth0 proto dhcp src 10.0.0.1 metric 100\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
open my $__fh, '>', "grep_p_routes2.txt" or die "Cannot write file: $!\n";  # line 36
print {$__fh} "default via 10.7.7.7 dev vpn_se proto static\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "-- functions.sh / softy: lookbehind -o extraction --\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$DEFAULT_ADAPTER = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); my $__sh_pv = do { my $__capf = "/tmp/sh2_cap_26_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { do { my @__pl = split(/\n/, do { my $__capf = "/tmp/sh2_cap_25_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { __sh_uu_run('grep', 'default', 'grep_p_routes.txt');$? = ($__sh_uu_rc) << 8; $CHILD_ERROR = $? >> 8;
 ; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }); pop @__pl if @__pl && $__pl[-1] eq ''; @__pl = @__pl[(@__pl - 1)..$#__pl] if @__pl > 1; say join("\n", @__pl) if @__pl; $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }; my $__sh_pv2 = do { my $__capf = "/tmp/sh2_cap_27_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { do { my $__g = $__sh_pv; my @__pl = split(/\n/, $__g); my $__m = 0; my $__c = 0; my $__r = qr/dev ([^[:space:]]+)/; for my $__l (@__pl) { while ($__l =~ /$__r/g) { print $&, "\n"; $__m = 1; } } $? = ($__m ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; }; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }; do { my @__o; for my $__l (split(/\n/, $__sh_pv2, -1)) { $__l =~ s{^dev }{}; push @__o, $__l; } print join("\n", @__o), (@__o ? "\n" : ""); $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
print "adapter=[" . ($DEFAULT_ADAPTER) . "]\n";  # line 40
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "-- jobs.sh: lookbehind -o + grep -v + head -1 --\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$ADAPTER = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); my $__sh_pv = do { my $__capf = "/tmp/sh2_cap_33_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { do { my $__g = do { my $__capf = "/tmp/sh2_cap_32_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { __sh_uu_run('grep', 'default', 'grep_p_routes2.txt');$? = ($__sh_uu_rc) << 8; $CHILD_ERROR = $? >> 8;
 ; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }; my @__pl = split(/\n/, $__g); my $__m = 0; my $__c = 0; my $__r = qr/dev ([^[:space:]]+)/; for my $__l (@__pl) { while ($__l =~ /$__r/g) { print $&, "\n"; $__m = 1; } } $? = ($__m ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; }; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }; my $__sh_pv2 = do { my $__capf = "/tmp/sh2_cap_34_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { do { my @__o; for my $__l (split(/\n/, $__sh_pv, -1)) { $__l =~ s{^dev }{}; push @__o, $__l; } print join("\n", @__o), (@__o ? "\n" : ""); $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }; my $__sh_pv3 = do { my $__capf = "/tmp/sh2_cap_35_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { do { my $__g = $__sh_pv2; my @__pl = split(/\n/, $__g); my $__m = 0; my $__c = 0; my $__r = qr/vpn_se/; for my $__l (@__pl) { if (!($__l =~ $__r)) { print $__l, "\n"; $__m = 1; } } $? = ($__m ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; }; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }; do { my @__pl = split(/\n/, $__sh_pv3); pop @__pl if @__pl && $__pl[-1] eq ''; @__pl = @__pl[0..0] if @__pl > 1; say join("\n", @__pl) if @__pl; $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
print("adapter2=[" . (((defined($ADAPTER) && length($ADAPTER)) ? $ADAPTER : "EMPTY")) . "]", "\n");  # line 44
$ADAPTER3 = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); my $__sh_pv = do { my $__capf = "/tmp/sh2_cap_41_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { do { my $__g = do { my $__capf = "/tmp/sh2_cap_40_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { __sh_uu_run('grep', 'default', 'grep_p_routes.txt', 'grep_p_routes2.txt');$? = ($__sh_uu_rc) << 8; $CHILD_ERROR = $? >> 8;
 ; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }; my @__pl = split(/\n/, $__g); my $__m = 0; my $__c = 0; my $__r = qr/dev ([^[:space:]]+)/; for my $__l (@__pl) { while ($__l =~ /$__r/g) { print $&, "\n"; $__m = 1; } } $? = ($__m ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; }; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }; my $__sh_pv2 = do { my $__capf = "/tmp/sh2_cap_42_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { do { my @__o; for my $__l (split(/\n/, $__sh_pv, -1)) { $__l =~ s{^dev }{}; push @__o, $__l; } print join("\n", @__o), (@__o ? "\n" : ""); $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }; my $__sh_pv3 = do { my $__capf = "/tmp/sh2_cap_43_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { do { my $__g = $__sh_pv2; my @__pl = split(/\n/, $__g); my $__m = 0; my $__c = 0; my $__r = qr/vpn_se/; for my $__l (@__pl) { if (!($__l =~ $__r)) { print $__l, "\n"; $__m = 1; } } $? = ($__m ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; }; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }; do { my @__pl = split(/\n/, $__sh_pv3); pop @__pl if @__pl && $__pl[-1] eq ''; @__pl = @__pl[0..0] if @__pl > 1; say join("\n", @__pl) if @__pl; $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
print("adapter3=[" . (((defined($ADAPTER3) && length($ADAPTER3)) ? $ADAPTER3 : "EMPTY")) . "]", "\n");  # line 46
print "-- snap-debug-info.sh: non-capturing group + \\s, -o --\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
open my $__fh, '>', "grep_p_snap.txt" or die "Cannot write file: $!\n";  # line 49
print {$__fh} "ID Status When\n1 Doing now\n2 Error now\n3 Doing now\n4 Error later\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$DOINGS = __sh_pipe_capture(q<'cat' 'grep_p_snap.txt' | 'tail' '-n' '+2' | 'grep' '-Eo' '([0-9]+[[:space:]]+Doing)' | 'awk' '{print $1}'>, "pipefail");
$ENV{DOINGS} = ${DOINGS} if exists $ENV{DOINGS};
print "doings=[" . ($DOINGS) . "]\n";  # line 51
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$ERRORS = __sh_pipe_capture(q<'cat' 'grep_p_snap.txt' | 'tail' '-n' '+2' | 'grep' '-Eo' '([0-9]+[[:space:]]+Error)' | 'awk' '{print $1}'>, "pipefail");
$ENV{ERRORS} = ${ERRORS} if exists $ENV{ERRORS};
print "errors=[" . ($ERRORS) . "]\n";  # line 53
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "-- softy: FQDN validation pattern (kept as grep -P) --\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$N = __sh_pipe_capture(q/'echo' 'sub.example.com' | 'grep' '-P' '-c' '(?=^.{1,254}$)(^(?>(?!\d+\.)[a-zA-Z0-9_\-]{1,63}\.?)+(?:[a-zA-Z]{2,})$)'/, "pipefail");  # line 62
$ENV{N} = ${N} if exists $ENV{N};
say "fqdn-count=$N";  # line 63
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$N = __sh_pipe_capture(q/'echo' '192.168.1.1' | 'grep' '-P' '-c' '(?=^.{1,254}$)(^(?>(?!\d+\.)[a-zA-Z0-9_\-]{1,63}\.?)+(?:[a-zA-Z]{2,})$)'/, "pipefail");  # line 64
$ENV{N} = ${N} if exists $ENV{N};
say "ip-count=$N";  # line 65
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "-- memtester.sh / terminateProcess.sh: pgrep -P (untouched) --\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$| = 1;  # line 68
my $__sh2_bg1 = fork();
$__sh2_last_bg = $__sh2_bg1;
if (defined $__sh2_bg1 && $__sh2_bg1 == 0) {
    sleep("0.01");$? = 0 << 8; $CHILD_ERROR = $? >> 8;
    exit $main_exit_code;
}
$BGPID = $__sh2_last_bg;  # line 69
$ENV{BGPID} = ${BGPID} if exists $ENV{BGPID};
$CPID = do { my $__c = __sh_pipe_capture(q{'pgrep' '-P' } . __sh_q($BGPID) . q{ 'no-such-name-xyz' 2>/dev/null}); $CHILD_ERROR = 0; $__c };  # line 70
print("pgrep-no-match=[" . (((defined($CPID) && length($CPID)) ? $CPID : "EMPTY")) . "]", "\n");  # line 71
$ENV{BGPID} = $BGPID;  # line 72
__sh_pipe_run(q{'wait' } . __sh_q($BGPID) . ' 2>/dev/null', "pipefail");
exit "0";  # line 74

exit $? >> 8;
