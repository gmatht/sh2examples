#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
use feature 'say';

our $CHILD_ERROR = 0;
my $filename;

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
# __SH_RUN_EXT_END

print "=== Basic Case Statement Example ===\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
if (($__sh_args[0]) =~ /^(?:start)$/) {  # line 9
    print "Starting the service...\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } elsif (($__sh_args[0]) =~ /^(?:stop)$/) {
    print "Stopping the service...\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } elsif (($__sh_args[0]) =~ /^(?:restart)$/) {
    print "Restarting the service...\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } else {
    print "Usage: " . ($0) . " {start|stop|restart}\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        exit "1";
}
print "=== Case Statement with Pattern Matching ===\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$filename = $__sh_args[1];  # line 28
if (($filename) =~ /^(?:.*\.txt)$/) {  # line 29
    say "Processing text file: $filename";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } elsif (($filename) =~ /^(?:.*\.sh)$/) {
    say "Processing shell script: $filename";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } elsif (($filename) =~ /^(?:.*\.py)$/) {
    say "Processing Python file: $filename";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } else {
    say "Unknown file type: $filename";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    }
print "=== Case Statement with Multiple Patterns ===\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
if (($__sh_args[2]) =~ /^(?:help|h|-h|--help)$/) {  # line 47
    print "Help information:\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        print "  start  - Start the service\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        print "  stop   - Stop the service\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        print "  status - Show service status\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } elsif (($__sh_args[2]) =~ /^(?:status|s|-s|--status)$/) {
    print "Service status: Running\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } else {
    print("Unknown option: " . ($__sh_args[2]), "\n");
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    }
print "=== Case Statement with Character Classes ===\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
if (($__sh_args[3]) =~ /^(?:[0-9])$/) {  # line 65
    print("Single digit: " . ($__sh_args[3]), "\n");
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } elsif (($__sh_args[3]) =~ /^(?:[a-z])$/) {
    print("Lowercase letter: " . ($__sh_args[3]), "\n");
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } elsif (($__sh_args[3]) =~ /^(?:[A-Z])$/) {
    print("Uppercase letter: " . ($__sh_args[3]), "\n");
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } elsif (($__sh_args[3]) =~ /^(?:[0-9][0-9])$/) {
    print("Two digit number: " . ($__sh_args[3]), "\n");
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } else {
    print("Other character: " . ($__sh_args[3]), "\n");
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    }
print "=== Case Statement with Default Action ===\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
if (($__sh_args[4]) =~ /^(?:red)$/) {  # line 86
    print "Color is red\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } elsif (($__sh_args[4]) =~ /^(?:green)$/) {
    print "Color is green\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } elsif (($__sh_args[4]) =~ /^(?:blue)$/) {
    print "Color is blue\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    }
print "=== Case Statement with Commands ===\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
if (($__sh_args[5]) =~ /^(?:ls)$/) {  # line 101
    __sh_uu_run('ls', '-l', '-a');
} elsif (($__sh_args[5]) =~ /^(?:date)$/) {
    __sh_pipe_run(q{'date'}, "pipefail");
} elsif (($__sh_args[5]) =~ /^(?:pwd)$/) {
    my $__sh_cap; do { $__sh_cap = (require Cwd, Cwd::getcwd()); $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; print $__sh_cap, "\n" if defined $__sh_cap;
} elsif (($__sh_args[5]) =~ /^(?:whoami)$/) {
    __sh_pipe_run(q{'whoami'}, "pipefail");
} else {
    print "Available commands: ls, date, pwd, whoami\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    }

exit $? >> 8;
