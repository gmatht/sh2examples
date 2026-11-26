#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';
use English qw(-no_match_vars $ERRNO $EVAL_ERROR $INPUT_RECORD_SEPARATOR $OS_ERROR $PROGRAM_NAME);

our $CHILD_ERROR = 0;
my $count;
my $matched;
my $lineno;
my $found;
my $longline;

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

print "== Basic grep parameters ==\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my @__gl = split(/\n/, 'text with pattern', -1); pop @__gl if @__gl && $__gl[-1] eq q{}; my @__mi; for my $__i (0..$#__gl) { if ($__gl[$__i] =~ /PATTERN/i) { push @__mi, $__i; } } my @__out; my %__se; for my $__i (@__mi) { my $__lo = $__i - 0; $__lo = 0 if $__lo < 0; my $__hi = $__i + 0; $__hi = $#__gl if $__hi > $#__gl; for my $__j ($__lo..$__hi) { push @__out, $__gl[$__j] unless $__se{$__j}++; } } if (@__out) { print join("\n", @__out), "\n"; } $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
print "  -i match: OK\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$count = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); my $__sh_pv = do { my $__capf = "/tmp/sh2_cap_5_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { do { my $__g = q{line1
line2
line3}; my @__pl = split(/\n/, $__g); my $__m = 0; my $__c = 0; my $__r = qr/line2/; for my $__l (@__pl) { if (!($__l =~ $__r)) { print $__l, "\n"; $__m = 1; } } $? = ($__m ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; }; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }; do { my @__pl = split(/\n/, $__sh_pv); print scalar(@__pl), "\n"; $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
print "  -v count: " . ($count) . " (expected 2)\n";  # line 10
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$matched = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); do { my $__g = q{match
no match
match again}; my @__pl = split(/\n/, $__g); my $__m = 0; my $__c = 0; my $__r = qr/match/; for my $__l (@__pl) { $__c++ if $__l =~ $__r; } print $__c, "\n"; $__m = $__c > 0 ? 1 : 0; $? = ($__m ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
print "  -c count: " . ($matched) . " (expected 2)\n";  # line 13
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "== Context parameters ==\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my @__gl = split(/\n/, q{line1
line2
TARGET
line4
line5}, -1); pop @__gl if @__gl && $__gl[-1] eq q{}; my @__mi; for my $__i (0..$#__gl) { if ($__gl[$__i] =~ /TARGET/) { push @__mi, $__i; } } my @__out; my %__se; for my $__i (@__mi) { my $__lo = $__i - 0; $__lo = 0 if $__lo < 0; my $__hi = $__i + 2; $__hi = $#__gl if $__hi > $#__gl; for my $__j ($__lo..$__hi) { push @__out, $__gl[$__j] unless $__se{$__j}++; } } open(my $__ofh, '>', '/tmp/grep_out.txt') or die "grep: '/tmp/grep_out.txt': $ERRNO"; if (@__out) { print $__ofh join("\n", @__out), "\n"; } close $__ofh; $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
print("  -A 2 lines: " . (do { my $__sh_cap; my $__f = '/tmp/grep_out.txt'; if (open(my $__h, '<', $__f)) { my $__d = do { local $/; <$__h> }; close($__h); $__d //= q{}; $__sh_cap = $__d =~ tr/\n//; $? = 0 << 8; $CHILD_ERROR = $? >> 8; } else { $__sh_cap = q{}; $? = 1 << 8; $CHILD_ERROR = $? >> 8; } $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap }) . " (expected 3)", "\n");  # line 17
do { my @__gl = split(/\n/, q{line1
line2
TARGET
line4
line5}, -1); pop @__gl if @__gl && $__gl[-1] eq q{}; my @__mi; for my $__i (0..$#__gl) { if ($__gl[$__i] =~ /TARGET/) { push @__mi, $__i; } } my @__out; my %__se; for my $__i (@__mi) { my $__lo = $__i - 2; $__lo = 0 if $__lo < 0; my $__hi = $__i + 0; $__hi = $#__gl if $__hi > $#__gl; for my $__j ($__lo..$__hi) { push @__out, $__gl[$__j] unless $__se{$__j}++; } } open(my $__ofh, '>', '/tmp/grep_out.txt') or die "grep: '/tmp/grep_out.txt': $ERRNO"; if (@__out) { print $__ofh join("\n", @__out), "\n"; } close $__ofh; $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
print("  -B 2 lines: " . (do { my $__sh_cap; my $__f = '/tmp/grep_out.txt'; if (open(my $__h, '<', $__f)) { my $__d = do { local $/; <$__h> }; close($__h); $__d //= q{}; $__sh_cap = $__d =~ tr/\n//; $? = 0 << 8; $CHILD_ERROR = $? >> 8; } else { $__sh_cap = q{}; $? = 1 << 8; $CHILD_ERROR = $? >> 8; } $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap }) . " (expected 3)", "\n");  # line 20
do { my @__gl = split(/\n/, q{line1
line2
TARGET
line4
line5}, -1); pop @__gl if @__gl && $__gl[-1] eq q{}; my @__mi; for my $__i (0..$#__gl) { if ($__gl[$__i] =~ /TARGET/) { push @__mi, $__i; } } my @__out; my %__se; for my $__i (@__mi) { my $__lo = $__i - 1; $__lo = 0 if $__lo < 0; my $__hi = $__i + 1; $__hi = $#__gl if $__hi > $#__gl; for my $__j ($__lo..$__hi) { push @__out, $__gl[$__j] unless $__se{$__j}++; } } open(my $__ofh, '>', '/tmp/grep_out.txt') or die "grep: '/tmp/grep_out.txt': $ERRNO"; if (@__out) { print $__ofh join("\n", @__out), "\n"; } close $__ofh; $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
print("  -C 1 lines: " . (do { my $__sh_cap; my $__f = '/tmp/grep_out.txt'; if (open(my $__h, '<', $__f)) { my $__d = do { local $/; <$__h> }; close($__h); $__d //= q{}; $__sh_cap = $__d =~ tr/\n//; $? = 0 << 8; $CHILD_ERROR = $? >> 8; } else { $__sh_cap = q{}; $? = 1 << 8; $CHILD_ERROR = $? >> 8; } $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap }) . " (expected 3)", "\n");  # line 23
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "== File handling parameters ==\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
open my $__fh, '>', "/tmp/grep_file.txt" or die "Cannot write file: $!\n";
print {$__fh} "content\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { print((sub { open(my $__gh, '<', '/tmp/grep_file.txt') or return 0; local $/; my $__gc = <$__gh>; close $__gh; my $__gn = 0; for my $__gl (split(/\n/, $__gc)) { $__gn++ if index($__gl, 'content') >= 0; } $__gn })->(), "\n"); if (((sub { open(my $__gh, '<', '/tmp/grep_file.txt') or return 0; local $/; my $__gc = <$__gh>; close $__gh; my $__gn = 0; for my $__gl (split(/\n/, $__gc)) { $__gn++ if index($__gl, 'content') >= 0; } $__gn })->() > 0)) { say '  -c file: OK'; $? = 0 << 8; $CHILD_ERROR = $? >> 8; } else { say '  -c file: FAIL'; $? = 0 << 8; $CHILD_ERROR = $? >> 8; } };
do { say '/tmp/grep_file.txt' if (sub { open(my $__gh, '<', '/tmp/grep_file.txt') or return 0; local $/; my $__gc = <$__gh>; close $__gh; my $__gn = 0; for my $__gl (split(/\n/, $__gc)) { $__gn++ if index($__gl, 'content') >= 0; } $__gn })->() > 0; if (((sub { open(my $__gh, '<', '/tmp/grep_file.txt') or return 0; local $/; my $__gc = <$__gh>; close $__gh; my $__gn = 0; for my $__gl (split(/\n/, $__gc)) { $__gn++ if index($__gl, 'content') >= 0; } $__gn })->() > 0)) { say '  -l: found'; $? = 0 << 8; $CHILD_ERROR = $? >> 8; } else { say '  -l: not found'; $? = 0 << 8; $CHILD_ERROR = $? >> 8; } };
do { say '/tmp/grep_file.txt' if (sub { open(my $__gh, '<', '/tmp/grep_file.txt') or return 0; local $/; my $__gc = <$__gh>; close $__gh; my $__gn = 0; for my $__gl (split(/\n/, $__gc)) { $__gn++ if index($__gl, 'nonexistent') >= 0; } $__gn })->() == 0; if (((sub { open(my $__gh, '<', '/tmp/grep_file.txt') or return 0; local $/; my $__gc = <$__gh>; close $__gh; my $__gn = 0; for my $__gl (split(/\n/, $__gc)) { $__gn++ if index($__gl, 'nonexistent') >= 0; } $__gn })->() > 0)) { say '  -L: not found (correct)'; $? = 0 << 8; $CHILD_ERROR = $? >> 8; } else { say '  -L: found (wrong)'; $? = 0 << 8; $CHILD_ERROR = $? >> 8; } };
print "== Output formatting parameters ==\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$matched = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     for my $__m (("text with pattern in it") =~ /pattern/g) { print $__m, "\n"; }$? = 0 << 8; $CHILD_ERROR = $? >> 8;
;; } $__cap =~ s/\n+\z//; $__cap };
print "  -o match: '" . ($matched) . "' (expected 'pattern')\n";  # line 33
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$lineno = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); my $__sh_pv = do { my $__capf = "/tmp/sh2_cap_7_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { do { my $__g = 'text with pattern in it'; my @__pl = split(/\n/, $__g); my $__m = 0; my $__c = 0; my $__r = qr/pattern/; for my $__i (0..$#__pl) { print(($__i + 1), ":", $__pl[$__i], "\n") if do { my $__l = $__pl[$__i]; $__l =~ $__r }; $__m = 1 if do { my $__l = $__pl[$__i]; $__l =~ $__r }; } $? = ($__m ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; }; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }; do { my @__o; for my $__l (split(/\n/, $__sh_pv, -1)) { if (index($__l, ':') >= 0) { my @__f = split(/\Q:\E/, $__l, -1); push @__o, @__f >= 1 ? $__f[1 - 1] : ''; } else { push @__o, $__l; } } print join("\n", @__o), (@__o ? "\n" : ""); $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
print "  -n line: " . ($lineno) . " (expected 1)\n";  # line 36
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "== Recursive parameters ==\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { require File::Path; File::Path::make_path('/tmp/grep_sub'); my $__ok = 1; $? = ($__ok ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; };
if ($? == 0) {
    open(my $__sh2_rfh1, '>', "/tmp/grep_sub/file.txt") or die "Cannot write to /tmp/grep_sub/file.txt: $!\n";
    my $__sh2_rsel1 = select($__sh2_rfh1);
    $| = 1;
    print "subfile content\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    select($__sh2_rsel1); close($__sh2_rfh1);
}
$found = __sh_pipe_capture(q{'grep' '-r' 'subfile' '/tmp/grep_sub' 2>/dev/null | 'wc' '-l'}, "pipefail");
print "  -r recursive: " . ($found) . " files matched (expected 1)\n";  # line 41
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { require File::Path; File::Path::remove_tree('/tmp/grep_sub'); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };  # line 42
print "== Line length parameters ==\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$longline = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     print("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
;; } $__cap =~ s/\n+\z//; $__cap };
do { if ($longline =~ /a/) { print STDOUT '  -m 1 (long line): OK', "\n"; } else { print STDOUT '  -m 1 (long line): FAIL', "\n"; } }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;print "== Word-regexp and line-regexp parameters ==\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my @__gl = split(/\n/, q{foo
foobar
bar}, -1); pop @__gl if @__gl && $__gl[-1] eq q{}; my @__mi; for my $__i (0..$#__gl) { if ($__gl[$__i] =~ /\bfoo\b/) { push @__mi, $__i; } } my @__out; my %__se; for my $__i (@__mi) { my $__lo = $__i - 0; $__lo = 0 if $__lo < 0; my $__hi = $__i + 0; $__hi = $#__gl if $__hi > $#__gl; for my $__j ($__lo..$__hi) { push @__out, $__gl[$__j] unless $__se{$__j}++; } } open(my $__ofh, '>', '/tmp/grep_out.txt') or die "grep: '/tmp/grep_out.txt': $ERRNO"; if (@__out) { print $__ofh join("\n", @__out), "\n"; } close $__ofh; $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
print("  -w word match lines: " . (do { my $__sh_cap; my $__f = '/tmp/grep_out.txt'; if (open(my $__h, '<', $__f)) { my $__d = do { local $/; <$__h> }; close($__h); $__d //= q{}; $__sh_cap = $__d =~ tr/\n//; $? = 0 << 8; $CHILD_ERROR = $? >> 8; } else { $__sh_cap = q{}; $? = 1 << 8; $CHILD_ERROR = $? >> 8; } $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap }) . " (expected 1)", "\n");  # line 50
do { unlink('/tmp/grep_file.txt', '/tmp/grep_out.txt'); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };  # line 52

exit $? >> 8;
