#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $main_exit_code = 0;
our $CHILD_ERROR = 0;

# __SH_RUN_EXT_BEGIN (UU-FFI.md external-command seam)
# uu-ffi .so present: in-process coreutils via uu_run (capture = temp-file
# fd-1 redirect around the in-process call — no pipe, so no concurrent-
# drain deadlock, and no C-side wrapper .so needed). Absent: guarded
# fork/exec fallback (one-time warning). The bash -c below is
# feature-guarded — check_qx strips this block before scanning.
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

$main_exit_code = $CHILD_ERROR = __sh_pipe_run(q['echo' 'test data' | 'awk' '
{
  while (getline < "/dev/null") {
    print "looping"
  }
}
' > '/dev/null'], "pipefail");
say "status: ok";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
