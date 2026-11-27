#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';
use English qw(-no_match_vars $ERRNO $EVAL_ERROR $INPUT_RECORD_SEPARATOR $OS_ERROR $PROGRAM_NAME);

our $CHILD_ERROR = 0;
my $file_content;
my $grep_result;
my $sed_result;
my $awk_result;
my $sort_result;
my $uniq_result;
my $word_count;
my $line_count;
my $head_result;
my $tail_result;
my $cut_result;
my $paste_result;
my $comm_result;
my $diff_result;
my $d;
my $tr_result;
my $xargs_result;

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
# __SH_RUN_EXT_END

print "=== Text Processing Commands ===\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$file_content = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n"; do { my @__lp = do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__f, '<', '000__04c_text_processing_commands.sh') or do { $CHILD_ERROR = 2; () }; my $__c = <$__f> // ''; close $__f; split(/\n/, $__c) }; pop @__lp if @__lp && $__lp[-1] eq q{}; if (@__lp > 5) { splice(@__lp, 5); } if (@__lp) { print join("\n", @__lp), "\n"; } }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;;; } $__cap =~ s/\n+\z//; $__cap };  # line 10
print "First 5 lines of this file:\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say $file_content;  # line 12
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$grep_result = do { my $_r = __sh_uu_capture('grep', '-n', 'echo', '000__04c_text_processing_commands.sh'); $CHILD_ERROR = $__sh_uu_rc; $_r =~ s/\n+\z//; $_r; };
print "Lines containing 'echo':\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say $grep_result;  # line 18
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$sed_result = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n"; do { my $__x = 'Hello World'; $__x =~ s{World}{Universe}; print STDOUT $__x, "\n"; }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;;; } $__cap =~ s/\n+\z//; $__cap };
say "Sed result: $sed_result";  # line 23
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$awk_result = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); do { my @__o; for my $__l (split(/\n/, '1 2 3 4 5')) { my @__F = split(/\s+/, $__l); push @__o, scalar(((0+($__F[0] // 0)) + (0+($__F[1] // 0)))); } print join("\n", @__o), (@__o ? "\n" : ""); $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
say "Awk sum result: $awk_result";  # line 28
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$sort_result = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); do { my @__pl = split(/\n/, q{zebra
apple
banana}); pop @__pl if @__pl && $__pl[$#__pl] eq ''; @__pl = sort { our ($a, $b); $a cmp $b } @__pl; say join("\n", @__pl); $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
print "Sorted words:\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say $sort_result;  # line 34
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$uniq_result = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); do { my @__u; for my $__l (split(/\n/, q{apple
apple
banana
banana
cherry})) { push @__u, $__l if !@__u || $__u[-1] ne $__l; } print join("\n", @__u), (@__u ? "\n" : ""); $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
print "Unique words:\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say $uniq_result;  # line 40
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$word_count = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); do { my $__n = 0; for my $__l (split(/\n/, 'Hello World')) { $__n += scalar(split(/\s+/, $__l)); } print $__n, "\n"; $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
$line_count = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); do { my @__pl = split(/\n/, q{line1
line2
line3}); print scalar(@__pl), "\n"; $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
say "Word count: $word_count";  # line 46
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "Line count: $line_count";  # line 47
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$head_result = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); do { my @__pl = split(/\n/, q{1
2
3
4
5
6
7
8
9
10}); pop @__pl if @__pl && $__pl[-1] eq ''; @__pl = @__pl[0..2] if @__pl > 3; say join("\n", @__pl) if @__pl; $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };  # line 51
say "First 3 numbers: $head_result";  # line 52
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$tail_result = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); do { my @__pl = split(/\n/, q{1
2
3
4
5
6
7
8
9
10}); pop @__pl if @__pl && $__pl[-1] eq ''; @__pl = @__pl[(@__pl - 3)..$#__pl] if @__pl > 3; say join("\n", @__pl) if @__pl; $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };  # line 56
say "Last 3 numbers: $tail_result";  # line 57
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$cut_result = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n"; do { my @__l = split(/
/, 'apple:banana:cherry', -1); pop @__l if @__l && $__l[-1] eq q{}; for my $__l (@__l) { my @__f = split(':', $__l); print STDOUT join(':', $__f[1]), "
"; } }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;;; } $__cap =~ s/\n+\z//; $__cap };
say "Second field: $cut_result";  # line 62
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
open my $__fh, '>', "temp1.txt" or die "Cannot write file: $!\n";
print {$__fh} "1\n2\n3\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
open my $__fh, '>', "temp2.txt" or die "Cannot write file: $!\n";
print {$__fh} "a\nb\nc\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$paste_result = do { my $_r = __sh_uu_capture('paste', 'temp1.txt', 'temp2.txt'); $CHILD_ERROR = $__sh_uu_rc; $_r =~ s/\n+\z//; $_r; };  # line 68
print "Pasted columns:\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say $paste_result;  # line 70
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { unlink('temp1.txt', 'temp2.txt'); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };  # line 71
open my $__fh, '>', "file1.txt" or die "Cannot write file: $!\n";
print {$__fh} "apple\nbanana\ncherry\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
open my $__fh, '>', "file2.txt" or die "Cannot write file: $!\n";
print {$__fh} "banana\ncherry\ndate\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$comm_result = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n"; do { my @__a = split(/\n/, do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__f, '<', 'file1.txt') or die "comm: 'file1.txt': $ERRNO"; my $__c = <$__f>; close $__f; $__c }, -1); pop @__a if @__a && $__a[-1] eq q{}; my @__b = split(/\n/, do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__g, '<', 'file2.txt') or die "comm: 'file2.txt': $ERRNO"; my $__c = <$__g>; close $__g; $__c }, -1); pop @__b if @__b && $__b[-1] eq q{}; my ($__i, $__j) = (0, 0); my @__out; while ($__i < @__a && $__j < @__b) { my $__c = $__a[$__i] cmp $__b[$__j]; if ($__c == 0) { push @__out, $__a[$__i]; $__i++; $__j++; } elsif ($__c < 0) { $__i++; } else { $__j++; } } if (@__out) { print STDOUT join("\n", @__out), "\n"; } }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;;; } $__cap =~ s/\n+\z//; $__cap };  # line 77
print "Common lines:\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say $comm_result;  # line 79
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$diff_result = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n"; my $__rc = 0; do { my @__a = split(/\n/, do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__f, '<', 'file1.txt') or die "diff: 'file1.txt': $ERRNO"; my $__c = <$__f>; close $__f; $__c }, -1); pop @__a if @__a && $__a[-1] eq q{}; my @__b = split(/\n/, do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__g, '<', 'file2.txt') or die "diff: 'file2.txt': $ERRNO"; my $__c = <$__g>; close $__g; $__c }, -1); pop @__b if @__b && $__b[-1] eq q{}; my @__dp; for my $__i (0..$#__a) { for my $__j (0..$#__b) { if ($__a[$__i] eq $__b[$__j]) { $__dp[$__i+1][$__j+1] = $__dp[$__i][$__j] + 1; } else { $__dp[$__i+1][$__j+1] = ($__dp[$__i][$__j+1] > $__dp[$__i+1][$__j]) ? $__dp[$__i][$__j+1] : $__dp[$__i+1][$__j]; } } } my ($__i, $__j) = ($#__a, $#__b); my @__ops; while ($__i >= 0 && $__j >= 0) { if ($__a[$__i] eq $__b[$__j]) { unshift @__ops, [0, $__i, $__j]; $__i--; $__j--; } elsif ($__dp[$__i][$__j+1] >= $__dp[$__i+1][$__j]) { unshift @__ops, [-1, $__i, $__j]; $__i--; } else { unshift @__ops, [1, $__i, $__j]; $__j--; } } while ($__i >= 0) { unshift @__ops, [-1, $__i, -1]; $__i--; } while ($__j >= 0) { unshift @__ops, [1, -1, $__j]; $__j--; }  my $__is_out = 0; my $__fh = \*STDOUT; my $__changed = 0; my $__idx = 0; my $__common = -1; while ($__idx < @__ops) { if ($__ops[$__idx][0] == 0) { $__common = $__ops[$__idx][1] + 1; $__idx++; next; } my @__dels; my @__adds; my $__last_a = -1; my $__last_b = -1; while ($__idx < @__ops && $__ops[$__idx][0] != 0) { if ($__ops[$__idx][0] == -1) { push @__dels, $__ops[$__idx][1]; $__last_a = $__ops[$__idx][1]; } else { push @__adds, $__ops[$__idx][2]; $__last_b = $__ops[$__idx][2]; } $__idx++; } $__changed = 1; my $__a1 = $__dels[0] + 1; my $__a2 = $__dels[-1] + 1; my $__b1 = $__adds[0] + 1; my $__b2 = $__adds[-1] + 1; my $__astr = $__a1 == $__a2 ? "$__a1" : "$__a1,$__a2"; my $__bstr = $__b1 == $__b2 ? "$__b1" : "$__b1,$__b2"; my $__opch = (@__dels && @__adds) ? "c" : (@__dels ? "d" : "a"); my $__hdr; if ($__opch eq "c") { $__hdr = "$__astr$__opch$__bstr"; } elsif ($__opch eq "d") { $__hdr = "$__astr$__opch" . (defined $__last_b ? $__last_b + 1 : 1); } else { my $__apos = $__common; $__hdr = "$__apos$__opch$__bstr"; } print $__fh "$__hdr\n"; for my $__k (@__dels) { print $__fh "< $__a[$__k]\n"; } if (@__dels && @__adds) { print $__fh "---\n"; } for my $__k (@__adds) { print $__fh "> $__b[$__k]\n"; } } $__rc = $__changed ? 1 : 0; close $__fh if $__is_out; }; $? = ($__rc) << 8; $CHILD_ERROR = $? >> 8; ;; } $__cap =~ s/\n+\z//; $__cap };  # line 83
print "File differences:\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say $diff_result;  # line 85
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$d = do { my $__sh_cap; $__sh_cap = do { require File::Temp; File::Temp::tempdir('sh2_ps_XXXXXX', TMPDIR => 1, CLEANUP => 0) }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 89
chdir($d) or do { warn "cd: " . $d . ": $!\n"; exit "1"; };  # line 90
$tr_result = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); do { my $__t = 'HELLO WORLD' . "\n"; $__t =~ tr/A-Z/a-z/; print $__t; $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
say "Lowercase: $tr_result";  # line 92
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$xargs_result = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); for my $__w (grep { length } split(/\s+/, '1 2 3')) { print(('Number:' . " " . $__w) . "\n"); } $? = 0 << 8; $CHILD_ERROR = $? >> 8;; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
print "Xargs result:\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say $xargs_result;  # line 98
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { unlink('file1.txt', 'file2.txt'); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };  # line 101
if (chdir("/")) {  # line 102
    
} else {
    warn "cd: " . "/" . ": $!\n";
    
}
do { require File::Path; File::Path::remove_tree($d); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };  # line 103
print "=== Text Processing Commands Complete ===\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
