#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';
use Carp;

our $CHILD_ERROR = 0;
my @PIPESTATUS;

my $__nocasematch = 0;
my $y;
my $z;
my @arr;
my $el;
my %aa;
my $k;
my $ax;
my $ly;
my $pv;
my $arr;
my $aa;

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
sub __sh_uu_capture {
    my @argv = @_;
    if ($__SH_UU && ($__SH_UU_OK{ $argv[0] // '' } // 0)) {
        # Capture = fd-1 redirect to a temp file around the IN-PROCESS
        # uu_run call (the fd redirect is what the C wrapper's drain
        # thread exists for; a temp file cannot fill, so no deadlock).
        my $tmpf = "/tmp/sh2_uu_${$}_" . ($__SH_UU_SEQ++) . ".cap";
        open(my $saved, '>&', \*STDOUT) or do { $__sh_uu_rc = 127; return ''; };
        open(STDOUT, '>', $tmpf) or do { $__sh_uu_rc = 127; return ''; };
        my $rc = eval { __sh_uu_run_ffi($argv[0], scalar(@argv), \@argv) };
        open(STDOUT, '>&', $saved) or do { $__sh_uu_rc = 127; return ''; };
        close($saved);
        $__sh_uu_rc = defined $rc ? $rc : 127;
        my $out = '';
        if (open(my $rf, '<', $tmpf)) { $out = do { local $/; <$rf> } // ''; close $rf; }
        unlink $tmpf;
        return $out;
    }
    my $out = do {
        open(my $fh, '-|', @argv) or do { $__sh_uu_rc = 127; return ''; };
        my $r = do { local $/; <$fh> };
        close $fh;
        $r;
    };
    $__sh_uu_rc = $? >> 8;
    return $out // '';
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

if (chdir("/tmp")) {  # line 14
    
} else {
    warn "cd: " . "/tmp" . ": $!\n";
    
}
$? = (('5' > '3') ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;  # line 17
say "arith:  $CHILD_ERROR";  # line 17
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = (((($__nocasematch) ? lc('abc') eq lc('abc') : 'abc' eq 'abc')) ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;  # line 18
say "str:    $CHILD_ERROR";  # line 18
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = ((-f '/etc/passwd') ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;  # line 19
say "file:   $CHILD_ERROR";  # line 19
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = ((('hello world' =~ '^hello')) ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;  # line 20
say "regex:  $CHILD_ERROR";  # line 20
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = ((!length('')) ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;  # line 21
say "empty:  $CHILD_ERROR";  # line 21
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = ((length('x')) ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;  # line 22
say "nonempty: $CHILD_ERROR";  # line 22
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = (('a' lt 'b') ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;  # line 23
say "lt:     $CHILD_ERROR";  # line 23
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = (((!((($__nocasematch) ? lc('a') eq lc('b') : 'a' eq 'b')))) ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;  # line 24
say "ne:    $CHILD_ERROR";  # line 24
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = ((!(-e '/no/such/file')) ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;  # line 25
say "notfile: $CHILD_ERROR";  # line 25
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "up=HELLO WORLD";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "down=hello world";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "firstup=HeLLo WoRLD";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$y = "one-two-one";
say "gsub=1-two-1";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "sub=1-two-one";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "prefix=two-one";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "suffix=one-two";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print("shortest=" . ((($y =~ s{^.*o}{}r))), "\n");  # line 40
print("greedy=" . ((($y =~ s{-.*$}{}r))), "\n");  # line 41
$z = "0123456789";
print("mid=" . (join(' ', "234")), "\n");
print("tail=" . (join(' ', substr($z, 5))), "\n");  # line 46
print("neg=" . (join(' ', substr($z, -3))), "\n");  # line 47
say "len=10";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
@arr = ("alpha", "beta", "gamma");  # line 52
print("idx=" . ($arr[1]), "\n");  # line 53
print("count=" . (join(' ', scalar(@arr))), "\n");
print("all=" . (join(' ', @arr)), "\n");  # line 55
for my $__el ((@arr)) {  # line 56
    $el = $__el;
    say "el=$el";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
%aa = ();  # line 60
$aa{'one'} = "1";  # line 61
$aa{'two'} = "2";  # line 62
$aa{'three'} = "3";  # line 63
print("aa-one=" . ($aa{'one'}), "\n");  # line 64
for my $__k (split /\s+/, do { __sh_pipe_capture(q{'echo' } . __sh_q(join(' ', sort keys %{aa})) . q{ | 'tr' ' ' '\n' | 'sort'}, "pipefail") }) {  # line 65
    $k = $__k;
    print("aa[" . ($k) . "]=" . ($aa{$k}), "\n");
    }
print "hello herestring\n";  # line 68
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
open(my $__sh2_rfh0, '>&', \*STDERR) or die "Cannot dup stderr: $!\n";
close(STDERR); open(STDERR, '>', "/tmp/gnu_both.txt") or die "Cannot write to /tmp/gnu_both.txt: $!\n";
print "both\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
open(STDERR, '>&', $__sh2_rfh0) or die "Cannot restore stderr: $!\n"; close($__sh2_rfh0);
open my $_fl_fh1, '<', "/tmp/gnu_both.txt"; unless (defined $_fl_fh1) { $! = 1; croak "open failed\n"; }
while (my $__cat_line1 = <$_fl_fh1>) {
    chomp $__cat_line1;
    say $__cat_line1;
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
{ local *STDOUT; open STDOUT, '>&', STDERR or die "Cannot dup STDOUT: $!\n";
    print "err-to-stderr\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
$ax = int(8);
say "arith=$ax";  # line 79
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$ly = int($ly=($ax * 2));
say "let=$ly";  # line 81
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
(($ax++));
say "postfix=$ax";  # line 83
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print("pow=" . (int((2 ** 10))), "\n");  # line 84
@PIPESTATUS = (1, 0); $? = (__sh_pipe_run(q{'false' | 'true'}, "pipefail")) << 8; $CHILD_ERROR = $? >> 8;
print("ps=" . ($PIPESTATUS[0]) . "," . ($PIPESTATUS[1]), "\n");  # line 88
print("no-nl");
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print ".\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "tab\there\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$pv = "value-42";
say $pv;  # line 96
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print("seq: " . (do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); do { my $__t = q{1
2
3} . "\n"; $__t =~ tr/\n/ /; print $__t; $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap }), "\n");  # line 99
__sh_uu_run('ln', '-s', '-f', '/etc/passwd', '/tmp/gnu_link');  # line 100
print("readlink-f: " . (do { my $_r = __sh_uu_capture('readlink', '-f', '/tmp/gnu_link'); $CHILD_ERROR = $__sh_uu_rc; $_r =~ s/\n+\z//; $_r; }), "\n");  # line 101
print("readlink-e: " . (do { my $_r = __sh_uu_capture('readlink', '-e', '/tmp/gnu_link'); $CHILD_ERROR = $__sh_uu_rc; $_r =~ s/\n+\z//; $_r; }), "\n");  # line 102
$? = (__sh_pipe_run(q{'readlink' '-m' '/no/such/path' > '/dev/null'}, "pipefail")) << 8; $CHILD_ERROR = $? >> 8;  # line 103
say "readlink-m rc=$CHILD_ERROR";  # line 103
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
open my $__fh, '>', "/tmp/gnu_a.txt" or die "Cannot write file: $!\n";  # line 105
print {$__fh} "abc\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
open my $__fh, '>', "/tmp/gnu_b.txt" or die "Cannot write file: $!\n";  # line 106
print {$__fh} "abd\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__c1 = (sub { open(my $__h, '<', '/tmp/gnu_a.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__c2 = (sub { open(my $__h, '<', '/tmp/gnu_b.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__n = length($__c1) < length($__c2) ? length($__c1) : length($__c2); my $__i = -1; for (my $__k = 0; $__k < $__n; $__k++) { if (substr($__c1,$__k,1) ne substr($__c2,$__k,1)) { $__i = $__k; last; } } my $__subj = length($__c1) < length($__c2) ? 1 : 2; my $__short = $__subj == 1 ? $__c1 : $__c2; my $__sn = $__subj == 1 ? '/tmp/gnu_a.txt' : '/tmp/gnu_b.txt'; my $__line = 1 + (() = substr($__c1, 0, $__i) =~ /\n/g); if ($__i >= 0) { my $__o1 = ord(substr($__c1,$__i,1)); my $__o2 = ord(substr($__c2,$__i,1)); printf("%s %s differ: byte %d, line %d is %o %s %o %s\n", '/tmp/gnu_a.txt', '/tmp/gnu_b.txt', $__i+1, $__line, $__o1, chr($__o1), $__o2, chr($__o2)); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__c1) == length($__c2)) { $? = 0 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__short) == 0) { printf STDERR ("cmp: EOF on %s which is empty\n", $__sn); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } else { my $__sl = 1 + (() = $__short =~ /\n/g); if (substr($__short, -1) eq "\n") { printf STDERR ("cmp: EOF on %s after byte %d, line %d\n", $__sn, length($__short), $__sl); } else { printf STDERR ("cmp: EOF on %s after byte %d, in line %d\n", $__sn, length($__short), $__sl); } $? = 1 << 8; $CHILD_ERROR = $? >> 8; } };  # line 107
say "cmp-b rc=$CHILD_ERROR";  # line 107
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__c1 = (sub { open(my $__h, '<', '/tmp/gnu_a.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__c2 = (sub { open(my $__h, '<', '/tmp/gnu_b.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); $__c1 = substr($__c1, 0, 2); $__c2 = substr($__c2, 0, 2); my $__n = length($__c1) < length($__c2) ? length($__c1) : length($__c2); my $__i = -1; for (my $__k = 0; $__k < $__n; $__k++) { if (substr($__c1,$__k,1) ne substr($__c2,$__k,1)) { $__i = $__k; last; } } my $__subj = length($__c1) < length($__c2) ? 1 : 2; my $__short = $__subj == 1 ? $__c1 : $__c2; my $__sn = $__subj == 1 ? '/tmp/gnu_a.txt' : '/tmp/gnu_b.txt'; my $__line = 1 + (() = substr($__c1, 0, $__i) =~ /\n/g); if ($__i >= 0) { printf("%s %s differ: byte %d, line %d\n", '/tmp/gnu_a.txt', '/tmp/gnu_b.txt', $__i+1, $__line); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__c1) == length($__c2)) { $? = 0 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__short) == 0) { printf STDERR ("cmp: EOF on %s which is empty\n", $__sn); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } else { my $__sl = 1 + (() = $__short =~ /\n/g); if (substr($__short, -1) eq "\n") { printf STDERR ("cmp: EOF on %s after byte %d, line %d\n", $__sn, length($__short), $__sl); } else { printf STDERR ("cmp: EOF on %s after byte %d, in line %d\n", $__sn, length($__short), $__sl); } $? = 1 << 8; $CHILD_ERROR = $? >> 8; } };  # line 108
say "cmp-n rc=$CHILD_ERROR";  # line 108
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__c1 = (sub { open(my $__h, '<', '/tmp/gnu_a.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__c2 = (sub { open(my $__h, '<', '/tmp/gnu_b.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); $__c1 = substr($__c1, 2); $__c2 = substr($__c2, 2); my $__n = length($__c1) < length($__c2) ? length($__c1) : length($__c2); my $__i = -1; for (my $__k = 0; $__k < $__n; $__k++) { if (substr($__c1,$__k,1) ne substr($__c2,$__k,1)) { $__i = $__k; last; } } my $__subj = length($__c1) < length($__c2) ? 1 : 2; my $__short = $__subj == 1 ? $__c1 : $__c2; my $__sn = $__subj == 1 ? '/tmp/gnu_a.txt' : '/tmp/gnu_b.txt'; my $__line = 1 + (() = substr($__c1, 0, $__i) =~ /\n/g); if ($__i >= 0) { printf("%s %s differ: byte %d, line %d\n", '/tmp/gnu_a.txt', '/tmp/gnu_b.txt', $__i+1, $__line); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__c1) == length($__c2)) { $? = 0 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__short) == 0) { printf STDERR ("cmp: EOF on %s which is empty\n", $__sn); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } else { my $__sl = 1 + (() = $__short =~ /\n/g); if (substr($__short, -1) eq "\n") { printf STDERR ("cmp: EOF on %s after byte %d, line %d\n", $__sn, length($__short), $__sl); } else { printf STDERR ("cmp: EOF on %s after byte %d, in line %d\n", $__sn, length($__short), $__sl); } $? = 1 << 8; $CHILD_ERROR = $? >> 8; } };  # line 109
say "cmp-i rc=$CHILD_ERROR";  # line 109
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
{
    my $__sh2_pcap = '';
    { local *STDOUT; open(STDOUT, '>', \$__sh2_pcap) or die "capture: $!\n";
        print("foo\nbar123\n");
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    ; }
    my $__sh2_ptail = q{'grep' '-E' '[0-9]+' > '/dev/null'};
    open(my $__sh2_pfh, '|-', 'bash', '-c', $__sh2_ptail, $0) or die "sh2perl: pipe open failed: $!\n";
    print $__sh2_pfh $__sh2_pcap;
    close $__sh2_pfh;
    $? = ($? >> 8) << 8; $CHILD_ERROR = $? >> 8;
}
say "grep-P rc=$CHILD_ERROR";  # line 111
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__x = 'a1b2c3'; $__x =~ s{[0-9]+}{N}g; print STDOUT $__x, "\n"; }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;my $__tr_out = do { my $__t = 'AbCdEf' . "\n"; $__t =~ tr/a-z/A-Z/; $__t }; print $__tr_out;
do { my $__t = '  spaced  ' . "\n"; $__t =~ tr/ / /s; print $__t; $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
{  # line 115
    my $__sh2_pcap = '';
    { local *STDOUT; open(STDOUT, '>', \$__sh2_pcap) or die "capture: $!\n";
        print("10K\n2M\n500\n");
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    ; }
    my $__sh2_ptail = q{'sort' '-h'};
    open(my $__sh2_pfh, '|-', 'bash', '-c', $__sh2_ptail, $0) or die "sh2perl: pipe open failed: $!\n";
    print $__sh2_pfh $__sh2_pcap;
    close $__sh2_pfh;
}
{
    my $__sh2_pcap = '';
    { local *STDOUT; open(STDOUT, '>', \$__sh2_pcap) or die "capture: $!\n";
        print("short\nvery-long-line-here\n");
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    ; }
    my $__sh2_ptail = q{'wc' '-L'};
    open(my $__sh2_pfh, '|-', 'bash', '-c', $__sh2_ptail, $0) or die "sh2perl: pipe open failed: $!\n";
    print $__sh2_pfh $__sh2_pcap;
    close $__sh2_pfh;
}
do { print(substr('abcdef', 0, 3));  };  # line 117
print "\n";  # line 117
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
{
    my $__sh2_pcap = '';
    { local *STDOUT; open(STDOUT, '>', \$__sh2_pcap) or die "capture: $!\n";
        print("abcdef");
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    ; }
    my $__sh2_ptail = q{'tail' '-c' '2'};
    open(my $__sh2_pfh, '|-', 'bash', '-c', $__sh2_ptail, $0) or die "sh2perl: pipe open failed: $!\n";
    print $__sh2_pfh $__sh2_pcap;
    close $__sh2_pfh;
}
print "\n";  # line 118
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$__nocasematch = 1;$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 121
if ((($__nocasematch) ? lc('ABC') eq lc('abc') : 'ABC' eq 'abc')) {
    print "nocase-match\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    }
$__nocasematch = 0;$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 123
if ((($__nocasematch) ? lc('ABC') eq lc('abc') : 'ABC' eq 'abc')) {
    print "case-matters\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } else {
    print "case-sensitive\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    }
do { unlink('/tmp/gnu_both.txt', '/tmp/gnu_link', '/tmp/gnu_a.txt', '/tmp/gnu_b.txt'); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };  # line 127

exit $? >> 8;
