#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';
use English qw(-no_match_vars $ERRNO $EVAL_ERROR $INPUT_RECORD_SEPARATOR $OS_ERROR $PROGRAM_NAME);

our $CHILD_ERROR = 0;
my $d;
my $a = '';
my $b = '';

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

$d = do { my $__sh_cap; $__sh_cap = do { require File::Temp; File::Temp::tempdir('sh2_ps_XXXXXX', TMPDIR => 1, CLEANUP => 0) }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 4
chdir($d) or do { exit "1"; };  # line 5
open my $__fh, '>', "file.txt" or die "Cannot write file: $!\n";  # line 6
print {$__fh} "apple\napple\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
open my $__fh, '>', "note.txt" or die "Cannot write file: $!\n";  # line 7
print {$__fh} "hello.txt\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
open my $__fh, '>', "s.sh" or die "Cannot write file: $!\n";  # line 8
print {$__fh} "function f() { echo hi; }\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
system('bash', '-c', q{'ls' | 'grep' "\.txt\$" | 'wc' '-l'});
print "\n";  # line 10
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my @__lp = do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__f, '<', 'file.txt') or do { $CHILD_ERROR = 2; () }; my $__c = <$__f> // ''; close $__f; split(/\n/, $__c) }; pop @__lp if @__lp && $__lp[-1] eq q{}; @__lp = sort { $a cmp $b } @__lp; my @__u; my @__n; for my $__l (@__lp) { if (@__u && $__u[-1] eq $__l) { $__n[-1]++; } else { push @__u, $__l; push @__n, 1; } } @__lp = map { sprintf('%7d %s', $__n[$_], $__u[$_]) } 0..$#__u; @__lp = sort { $a <=> $b } @__lp; @__lp = reverse @__lp; if (@__lp) { print join("\n", @__lp), "\n"; } }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;print "\n";  # line 12
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
__sh_pipe_run(q{'find' '.' '-name' '*.sh' | 'xargs' 'grep' '-l' 'function' | 'tr' '-d' '\\/'}, "pipefail");
print "\n";  # line 14
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
my $__sh_pv = do { my $__capf = "/tmp/sh2_cap_1_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { do { my $__t = do { my $__sh_cap; if (open(my $__h, '<', 'file.txt')) { local $/; my $__d = <$__h>; close($__h); $__sh_cap = defined($__d) ? $__d : q{}; $? = 0 << 8; $CHILD_ERROR = $? >> 8; } else { $__sh_cap = q{}; $? = 1 << 8; $CHILD_ERROR = $? >> 8; } $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap } . "\n"; $__t =~ tr/a/b/; print $__t; $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }; do { my $__g = $__sh_pv; my $__m = 0; for my $__l (split(/\n/, $__g)) { if ($__l =~ /hello/) { print $__l, "\n"; $__m = 1; } } $? = ($__m ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; };
print "\n";  # line 16
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my @__lp = do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__f, '<', 'file.txt') or do { $CHILD_ERROR = 2; () }; my $__c = <$__f> // ''; close $__f; split(/\n/, $__c) }; pop @__lp if @__lp && $__lp[-1] eq q{}; @__lp = sort { $a cmp $b } @__lp; @__lp = grep { /hello/ } @__lp; if (@__lp) { print join("\n", @__lp), "\n"; } }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;chdir("/") or die "cd: $!\n";  # line 18
do { require File::Path; File::Path::remove_tree($d); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };  # line 19

exit $? >> 8;
