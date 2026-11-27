#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;
use feature 'say';

my $__ps_tmp0;
my $__ps_tmp1;
my $__ps_status0;
my $__ps_tmp2;
my $__ps_tmp3;

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
# __SH_RUN_EXT_END

open my $__fh, '>', "/tmp/cmp_a.txt" or die "Cannot write file: $!\n";
print {$__fh} "abcdefghij\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
open my $__fh, '>', "/tmp/cmp_same.txt" or die "Cannot write file: $!\n";
print {$__fh} "abcdefghij\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
open my $__fh, '>', "/tmp/cmp_diff.txt" or die "Cannot write file: $!\n";
print {$__fh} "abcdeZghij\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
open my $__fh, '>', "/tmp/cmp_diff2.txt" or die "Cannot write file: $!\n";
print {$__fh} "xyzdefghij\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
open my $__fh, '>', "/tmp/cmp_short.txt" or die "Cannot write file: $!\n";
print {$__fh} "abc\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__ok = 1; for my $__t ('/tmp/cmp_empty.txt') { if (open(my $__h, '>>', $__t)) { close($__h); } else { $__ok = 0; print STDERR "touch: cannot touch '$__t': $!\n"; } } $? = ($__ok ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; };  # line 11
do { my $__c1 = (sub { open(my $__h, '<', '/tmp/cmp_a.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__c2 = (sub { open(my $__h, '<', '/tmp/cmp_same.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__n = length($__c1) < length($__c2) ? length($__c1) : length($__c2); my $__i = -1; for (my $__k = 0; $__k < $__n; $__k++) { if (substr($__c1,$__k,1) ne substr($__c2,$__k,1)) { $__i = $__k; last; } } my $__subj = length($__c1) < length($__c2) ? 1 : 2; my $__short = $__subj == 1 ? $__c1 : $__c2; my $__sn = $__subj == 1 ? '/tmp/cmp_a.txt' : '/tmp/cmp_same.txt'; my $__line = 1 + (() = substr($__c1, 0, $__i) =~ /\n/g); if ($__i >= 0) { printf("%s %s differ: byte %d, line %d\n", '/tmp/cmp_a.txt', '/tmp/cmp_same.txt', $__i+1, $__line); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__c1) == length($__c2)) { $? = 0 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__short) == 0) { printf STDERR ("cmp: EOF on %s which is empty\n", $__sn); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } else { my $__sl = 1 + (() = $__short =~ /\n/g); if (substr($__short, -1) eq "\n") { printf STDERR ("cmp: EOF on %s after byte %d, line %d\n", $__sn, length($__short), $__sl); } else { printf STDERR ("cmp: EOF on %s after byte %d, in line %d\n", $__sn, length($__short), $__sl); } $? = 1 << 8; $CHILD_ERROR = $? >> 8; } };  # line 16
say "exit: $CHILD_ERROR";  # line 17
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__c1 = (sub { open(my $__h, '<', '/tmp/cmp_a.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__c2 = (sub { open(my $__h, '<', '/tmp/cmp_diff.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__n = length($__c1) < length($__c2) ? length($__c1) : length($__c2); my $__i = -1; for (my $__k = 0; $__k < $__n; $__k++) { if (substr($__c1,$__k,1) ne substr($__c2,$__k,1)) { $__i = $__k; last; } } my $__subj = length($__c1) < length($__c2) ? 1 : 2; my $__short = $__subj == 1 ? $__c1 : $__c2; my $__sn = $__subj == 1 ? '/tmp/cmp_a.txt' : '/tmp/cmp_diff.txt'; my $__line = 1 + (() = substr($__c1, 0, $__i) =~ /\n/g); if ($__i >= 0) { printf("%s %s differ: byte %d, line %d\n", '/tmp/cmp_a.txt', '/tmp/cmp_diff.txt', $__i+1, $__line); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__c1) == length($__c2)) { $? = 0 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__short) == 0) { printf STDERR ("cmp: EOF on %s which is empty\n", $__sn); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } else { my $__sl = 1 + (() = $__short =~ /\n/g); if (substr($__short, -1) eq "\n") { printf STDERR ("cmp: EOF on %s after byte %d, line %d\n", $__sn, length($__short), $__sl); } else { printf STDERR ("cmp: EOF on %s after byte %d, in line %d\n", $__sn, length($__short), $__sl); } $? = 1 << 8; $CHILD_ERROR = $? >> 8; } };  # line 20
say "exit: $CHILD_ERROR";  # line 21
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__c1 = (sub { open(my $__h, '<', '/tmp/cmp_a.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__c2 = (sub { open(my $__h, '<', '/tmp/cmp_empty.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__n = length($__c1) < length($__c2) ? length($__c1) : length($__c2); my $__i = -1; for (my $__k = 0; $__k < $__n; $__k++) { if (substr($__c1,$__k,1) ne substr($__c2,$__k,1)) { $__i = $__k; last; } } my $__subj = length($__c1) < length($__c2) ? 1 : 2; my $__short = $__subj == 1 ? $__c1 : $__c2; my $__sn = $__subj == 1 ? '/tmp/cmp_a.txt' : '/tmp/cmp_empty.txt'; my $__line = 1 + (() = substr($__c1, 0, $__i) =~ /\n/g); if ($__i >= 0) { printf("%s %s differ: byte %d, line %d\n", '/tmp/cmp_a.txt', '/tmp/cmp_empty.txt', $__i+1, $__line); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__c1) == length($__c2)) { $? = 0 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__short) == 0) { printf STDERR ("cmp: EOF on %s which is empty\n", $__sn); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } else { my $__sl = 1 + (() = $__short =~ /\n/g); if (substr($__short, -1) eq "\n") { printf STDERR ("cmp: EOF on %s after byte %d, line %d\n", $__sn, length($__short), $__sl); } else { printf STDERR ("cmp: EOF on %s after byte %d, in line %d\n", $__sn, length($__short), $__sl); } $? = 1 << 8; $CHILD_ERROR = $? >> 8; } };  # line 24
say "exit: $CHILD_ERROR";  # line 25
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__c1 = (sub { open(my $__h, '<', '/tmp/cmp_empty.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__c2 = (sub { open(my $__h, '<', '/tmp/cmp_empty.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__n = length($__c1) < length($__c2) ? length($__c1) : length($__c2); my $__i = -1; for (my $__k = 0; $__k < $__n; $__k++) { if (substr($__c1,$__k,1) ne substr($__c2,$__k,1)) { $__i = $__k; last; } } my $__subj = length($__c1) < length($__c2) ? 1 : 2; my $__short = $__subj == 1 ? $__c1 : $__c2; my $__sn = $__subj == 1 ? '/tmp/cmp_empty.txt' : '/tmp/cmp_empty.txt'; my $__line = 1 + (() = substr($__c1, 0, $__i) =~ /\n/g); if ($__i >= 0) { printf("%s %s differ: byte %d, line %d\n", '/tmp/cmp_empty.txt', '/tmp/cmp_empty.txt', $__i+1, $__line); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__c1) == length($__c2)) { $? = 0 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__short) == 0) { printf STDERR ("cmp: EOF on %s which is empty\n", $__sn); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } else { my $__sl = 1 + (() = $__short =~ /\n/g); if (substr($__short, -1) eq "\n") { printf STDERR ("cmp: EOF on %s after byte %d, line %d\n", $__sn, length($__short), $__sl); } else { printf STDERR ("cmp: EOF on %s after byte %d, in line %d\n", $__sn, length($__short), $__sl); } $? = 1 << 8; $CHILD_ERROR = $? >> 8; } };  # line 28
say "exit: $CHILD_ERROR";  # line 29
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__c1 = (sub { open(my $__h, '<', '/tmp/cmp_a.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__c2 = (sub { open(my $__h, '<', '/tmp/cmp_diff.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__n = length($__c1) < length($__c2) ? length($__c1) : length($__c2); my $__i = -1; for (my $__k = 0; $__k < $__n; $__k++) { if (substr($__c1,$__k,1) ne substr($__c2,$__k,1)) { $__i = $__k; last; } } $? = (($__i < 0 && length($__c1) == length($__c2) ? 0 : 1)) << 8; $CHILD_ERROR = $? >> 8; };  # line 32
say "-s exit: $CHILD_ERROR";  # line 33
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__c1 = (sub { open(my $__h, '<', '/tmp/cmp_a.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__c2 = (sub { open(my $__h, '<', '/tmp/cmp_same.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__n = length($__c1) < length($__c2) ? length($__c1) : length($__c2); my $__i = -1; for (my $__k = 0; $__k < $__n; $__k++) { if (substr($__c1,$__k,1) ne substr($__c2,$__k,1)) { $__i = $__k; last; } } $? = (($__i < 0 && length($__c1) == length($__c2) ? 0 : 1)) << 8; $CHILD_ERROR = $? >> 8; };  # line 34
say "-s same exit: $CHILD_ERROR";  # line 35
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__c1 = (sub { open(my $__h, '<', '/tmp/cmp_a.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__c2 = (sub { open(my $__h, '<', '/tmp/cmp_diff.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__n = length($__c1) < length($__c2) ? length($__c1) : length($__c2); my $__i = -1; for (my $__k = 0; $__k < $__n; $__k++) { if (substr($__c1,$__k,1) ne substr($__c2,$__k,1)) { $__i = $__k; last; } } my $__subj = length($__c1) < length($__c2) ? 1 : 2; my $__short = $__subj == 1 ? $__c1 : $__c2; my $__sn = $__subj == 1 ? '/tmp/cmp_a.txt' : '/tmp/cmp_diff.txt'; my $__diff = 0; for (my $__k = 0; $__k < $__n; $__k++) { if (substr($__c1,$__k,1) ne substr($__c2,$__k,1)) { $__diff = 1; printf(" %d %o %o\n", $__k+1, ord(substr($__c1,$__k,1)), ord(substr($__c2,$__k,1))); } } if (length($__c1) != length($__c2)) { $__diff = 1; } $? = (($__diff ? 1 : 0)) << 8; $CHILD_ERROR = $? >> 8; };  # line 38
say "-l exit: $CHILD_ERROR";  # line 39
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__c1 = (sub { open(my $__h, '<', '/tmp/cmp_a.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__c2 = (sub { open(my $__h, '<', '/tmp/cmp_diff.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__n = length($__c1) < length($__c2) ? length($__c1) : length($__c2); my $__i = -1; for (my $__k = 0; $__k < $__n; $__k++) { if (substr($__c1,$__k,1) ne substr($__c2,$__k,1)) { $__i = $__k; last; } } my $__subj = length($__c1) < length($__c2) ? 1 : 2; my $__short = $__subj == 1 ? $__c1 : $__c2; my $__sn = $__subj == 1 ? '/tmp/cmp_a.txt' : '/tmp/cmp_diff.txt'; my $__line = 1 + (() = substr($__c1, 0, $__i) =~ /\n/g); if ($__i >= 0) { my $__o1 = ord(substr($__c1,$__i,1)); my $__o2 = ord(substr($__c2,$__i,1)); printf("%s %s differ: byte %d, line %d is %o %s %o %s\n", '/tmp/cmp_a.txt', '/tmp/cmp_diff.txt', $__i+1, $__line, $__o1, chr($__o1), $__o2, chr($__o2)); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__c1) == length($__c2)) { $? = 0 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__short) == 0) { printf STDERR ("cmp: EOF on %s which is empty\n", $__sn); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } else { my $__sl = 1 + (() = $__short =~ /\n/g); if (substr($__short, -1) eq "\n") { printf STDERR ("cmp: EOF on %s after byte %d, line %d\n", $__sn, length($__short), $__sl); } else { printf STDERR ("cmp: EOF on %s after byte %d, in line %d\n", $__sn, length($__short), $__sl); } $? = 1 << 8; $CHILD_ERROR = $? >> 8; } };  # line 42
say "-b exit: $CHILD_ERROR";  # line 43
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__c1 = (sub { open(my $__h, '<', '/tmp/cmp_a.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__c2 = (sub { open(my $__h, '<', '/tmp/cmp_diff.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); $__c1 = substr($__c1, 0, 5); $__c2 = substr($__c2, 0, 5); my $__n = length($__c1) < length($__c2) ? length($__c1) : length($__c2); my $__i = -1; for (my $__k = 0; $__k < $__n; $__k++) { if (substr($__c1,$__k,1) ne substr($__c2,$__k,1)) { $__i = $__k; last; } } my $__subj = length($__c1) < length($__c2) ? 1 : 2; my $__short = $__subj == 1 ? $__c1 : $__c2; my $__sn = $__subj == 1 ? '/tmp/cmp_a.txt' : '/tmp/cmp_diff.txt'; my $__line = 1 + (() = substr($__c1, 0, $__i) =~ /\n/g); if ($__i >= 0) { printf("%s %s differ: byte %d, line %d\n", '/tmp/cmp_a.txt', '/tmp/cmp_diff.txt', $__i+1, $__line); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__c1) == length($__c2)) { $? = 0 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__short) == 0) { printf STDERR ("cmp: EOF on %s which is empty\n", $__sn); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } else { my $__sl = 1 + (() = $__short =~ /\n/g); if (substr($__short, -1) eq "\n") { printf STDERR ("cmp: EOF on %s after byte %d, line %d\n", $__sn, length($__short), $__sl); } else { printf STDERR ("cmp: EOF on %s after byte %d, in line %d\n", $__sn, length($__short), $__sl); } $? = 1 << 8; $CHILD_ERROR = $? >> 8; } };  # line 47
say "-n 5 exit: $CHILD_ERROR";  # line 48
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__c1 = (sub { open(my $__h, '<', '/tmp/cmp_a.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__c2 = (sub { open(my $__h, '<', '/tmp/cmp_diff.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); $__c1 = substr($__c1, 0, 10); $__c2 = substr($__c2, 0, 10); my $__n = length($__c1) < length($__c2) ? length($__c1) : length($__c2); my $__i = -1; for (my $__k = 0; $__k < $__n; $__k++) { if (substr($__c1,$__k,1) ne substr($__c2,$__k,1)) { $__i = $__k; last; } } my $__subj = length($__c1) < length($__c2) ? 1 : 2; my $__short = $__subj == 1 ? $__c1 : $__c2; my $__sn = $__subj == 1 ? '/tmp/cmp_a.txt' : '/tmp/cmp_diff.txt'; my $__line = 1 + (() = substr($__c1, 0, $__i) =~ /\n/g); if ($__i >= 0) { printf("%s %s differ: byte %d, line %d\n", '/tmp/cmp_a.txt', '/tmp/cmp_diff.txt', $__i+1, $__line); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__c1) == length($__c2)) { $? = 0 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__short) == 0) { printf STDERR ("cmp: EOF on %s which is empty\n", $__sn); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } else { my $__sl = 1 + (() = $__short =~ /\n/g); if (substr($__short, -1) eq "\n") { printf STDERR ("cmp: EOF on %s after byte %d, line %d\n", $__sn, length($__short), $__sl); } else { printf STDERR ("cmp: EOF on %s after byte %d, in line %d\n", $__sn, length($__short), $__sl); } $? = 1 << 8; $CHILD_ERROR = $? >> 8; } };  # line 50
say "-n 10 exit: $CHILD_ERROR";  # line 51
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__c1 = (sub { open(my $__h, '<', '/tmp/cmp_a.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__c2 = (sub { open(my $__h, '<', '/tmp/cmp_short.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); $__c1 = substr($__c1, 0, 10); $__c2 = substr($__c2, 0, 10); my $__n = length($__c1) < length($__c2) ? length($__c1) : length($__c2); my $__i = -1; for (my $__k = 0; $__k < $__n; $__k++) { if (substr($__c1,$__k,1) ne substr($__c2,$__k,1)) { $__i = $__k; last; } } my $__subj = length($__c1) < length($__c2) ? 1 : 2; my $__short = $__subj == 1 ? $__c1 : $__c2; my $__sn = $__subj == 1 ? '/tmp/cmp_a.txt' : '/tmp/cmp_short.txt'; my $__line = 1 + (() = substr($__c1, 0, $__i) =~ /\n/g); if ($__i >= 0) { printf("%s %s differ: byte %d, line %d\n", '/tmp/cmp_a.txt', '/tmp/cmp_short.txt', $__i+1, $__line); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__c1) == length($__c2)) { $? = 0 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__short) == 0) { printf STDERR ("cmp: EOF on %s which is empty\n", $__sn); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } else { my $__sl = 1 + (() = $__short =~ /\n/g); if (substr($__short, -1) eq "\n") { printf STDERR ("cmp: EOF on %s after byte %d, line %d\n", $__sn, length($__short), $__sl); } else { printf STDERR ("cmp: EOF on %s after byte %d, in line %d\n", $__sn, length($__short), $__sl); } $? = 1 << 8; $CHILD_ERROR = $? >> 8; } };  # line 53
say "-n 10 short exit: $CHILD_ERROR";  # line 54
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__c1 = (sub { open(my $__h, '<', '/tmp/cmp_a.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__c2 = (sub { open(my $__h, '<', '/tmp/cmp_diff.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); $__c1 = substr($__c1, 6); $__c2 = substr($__c2, 6); my $__n = length($__c1) < length($__c2) ? length($__c1) : length($__c2); my $__i = -1; for (my $__k = 0; $__k < $__n; $__k++) { if (substr($__c1,$__k,1) ne substr($__c2,$__k,1)) { $__i = $__k; last; } } my $__subj = length($__c1) < length($__c2) ? 1 : 2; my $__short = $__subj == 1 ? $__c1 : $__c2; my $__sn = $__subj == 1 ? '/tmp/cmp_a.txt' : '/tmp/cmp_diff.txt'; my $__line = 1 + (() = substr($__c1, 0, $__i) =~ /\n/g); if ($__i >= 0) { printf("%s %s differ: byte %d, line %d\n", '/tmp/cmp_a.txt', '/tmp/cmp_diff.txt', $__i+1, $__line); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__c1) == length($__c2)) { $? = 0 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__short) == 0) { printf STDERR ("cmp: EOF on %s which is empty\n", $__sn); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } else { my $__sl = 1 + (() = $__short =~ /\n/g); if (substr($__short, -1) eq "\n") { printf STDERR ("cmp: EOF on %s after byte %d, line %d\n", $__sn, length($__short), $__sl); } else { printf STDERR ("cmp: EOF on %s after byte %d, in line %d\n", $__sn, length($__short), $__sl); } $? = 1 << 8; $CHILD_ERROR = $? >> 8; } };  # line 58
say "-i 6 exit: $CHILD_ERROR";  # line 59
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__c1 = (sub { open(my $__h, '<', '/tmp/cmp_a.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__c2 = (sub { open(my $__h, '<', '/tmp/cmp_diff.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); $__c1 = substr($__c1, 3); $__c2 = substr($__c2, 3); my $__n = length($__c1) < length($__c2) ? length($__c1) : length($__c2); my $__i = -1; for (my $__k = 0; $__k < $__n; $__k++) { if (substr($__c1,$__k,1) ne substr($__c2,$__k,1)) { $__i = $__k; last; } } my $__subj = length($__c1) < length($__c2) ? 1 : 2; my $__short = $__subj == 1 ? $__c1 : $__c2; my $__sn = $__subj == 1 ? '/tmp/cmp_a.txt' : '/tmp/cmp_diff.txt'; my $__line = 1 + (() = substr($__c1, 0, $__i) =~ /\n/g); if ($__i >= 0) { printf("%s %s differ: byte %d, line %d\n", '/tmp/cmp_a.txt', '/tmp/cmp_diff.txt', $__i+1, $__line); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__c1) == length($__c2)) { $? = 0 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__short) == 0) { printf STDERR ("cmp: EOF on %s which is empty\n", $__sn); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } else { my $__sl = 1 + (() = $__short =~ /\n/g); if (substr($__short, -1) eq "\n") { printf STDERR ("cmp: EOF on %s after byte %d, line %d\n", $__sn, length($__short), $__sl); } else { printf STDERR ("cmp: EOF on %s after byte %d, in line %d\n", $__sn, length($__short), $__sl); } $? = 1 << 8; $CHILD_ERROR = $? >> 8; } };  # line 61
say "-i 3 exit: $CHILD_ERROR";  # line 62
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__c1 = (sub { open(my $__h, '<', '/tmp/cmp_a.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__c2 = (sub { open(my $__h, '<', '/tmp/cmp_diff.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); $__c1 = substr($__c1, 0); $__c2 = substr($__c2, 6); my $__n = length($__c1) < length($__c2) ? length($__c1) : length($__c2); my $__i = -1; for (my $__k = 0; $__k < $__n; $__k++) { if (substr($__c1,$__k,1) ne substr($__c2,$__k,1)) { $__i = $__k; last; } } my $__subj = length($__c1) < length($__c2) ? 1 : 2; my $__short = $__subj == 1 ? $__c1 : $__c2; my $__sn = $__subj == 1 ? '/tmp/cmp_a.txt' : '/tmp/cmp_diff.txt'; my $__line = 1 + (() = substr($__c1, 0, $__i) =~ /\n/g); if ($__i >= 0) { printf("%s %s differ: byte %d, line %d\n", '/tmp/cmp_a.txt', '/tmp/cmp_diff.txt', $__i+1, $__line); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__c1) == length($__c2)) { $? = 0 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__short) == 0) { printf STDERR ("cmp: EOF on %s which is empty\n", $__sn); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } else { my $__sl = 1 + (() = $__short =~ /\n/g); if (substr($__short, -1) eq "\n") { printf STDERR ("cmp: EOF on %s after byte %d, line %d\n", $__sn, length($__short), $__sl); } else { printf STDERR ("cmp: EOF on %s after byte %d, in line %d\n", $__sn, length($__short), $__sl); } $? = 1 << 8; $CHILD_ERROR = $? >> 8; } };  # line 66
say "-i 0:6 exit: $CHILD_ERROR";  # line 67
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__c1 = (sub { open(my $__h, '<', '/tmp/cmp_a.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); my $__c2 = (sub { open(my $__h, '<', '/tmp/cmp_diff2.txt') or return ''; local $/; my $c = <$__h>; close $__h; $c })->(); $__c1 = substr($__c1, 5); $__c2 = substr($__c2, 0); my $__n = length($__c1) < length($__c2) ? length($__c1) : length($__c2); my $__i = -1; for (my $__k = 0; $__k < $__n; $__k++) { if (substr($__c1,$__k,1) ne substr($__c2,$__k,1)) { $__i = $__k; last; } } my $__subj = length($__c1) < length($__c2) ? 1 : 2; my $__short = $__subj == 1 ? $__c1 : $__c2; my $__sn = $__subj == 1 ? '/tmp/cmp_a.txt' : '/tmp/cmp_diff2.txt'; my $__line = 1 + (() = substr($__c1, 0, $__i) =~ /\n/g); if ($__i >= 0) { printf("%s %s differ: byte %d, line %d\n", '/tmp/cmp_a.txt', '/tmp/cmp_diff2.txt', $__i+1, $__line); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__c1) == length($__c2)) { $? = 0 << 8; $CHILD_ERROR = $? >> 8; } elsif (length($__short) == 0) { printf STDERR ("cmp: EOF on %s which is empty\n", $__sn); $? = 1 << 8; $CHILD_ERROR = $? >> 8; } else { my $__sl = 1 + (() = $__short =~ /\n/g); if (substr($__short, -1) eq "\n") { printf STDERR ("cmp: EOF on %s after byte %d, line %d\n", $__sn, length($__short), $__sl); } else { printf STDERR ("cmp: EOF on %s after byte %d, in line %d\n", $__sn, length($__short), $__sl); } $? = 1 << 8; $CHILD_ERROR = $? >> 8; } };  # line 69
say "-i 5:0 exit: $CHILD_ERROR";  # line 70
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$__ps_tmp0 = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
open my $__fh, '>', $__ps_tmp0 or die "Cannot write file: $!\n";
print {$__fh} "a\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$__ps_tmp1 = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
open my $__fh, '>', $__ps_tmp1 or die "Cannot write file: $!\n";
print {$__fh} "a\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
__sh_uu_run('cmp', '-s', $__ps_tmp0, $__ps_tmp1);$? = ($__sh_uu_rc) << 8; $CHILD_ERROR = $? >> 8;
$__ps_status0 = $CHILD_ERROR;
do { unlink($__ps_tmp0, $__ps_tmp1); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
$CHILD_ERROR = $__ps_status0;
say "aa -s exit: $CHILD_ERROR";  # line 74
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$__ps_tmp2 = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
open my $__fh, '>', $__ps_tmp2 or die "Cannot write file: $!\n";
print {$__fh} "b\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$__ps_tmp3 = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
open my $__fh, '>', $__ps_tmp3 or die "Cannot write file: $!\n";
print {$__fh} "c\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
__sh_uu_run('cmp', '-s', $__ps_tmp2, $__ps_tmp3);$? = ($__sh_uu_rc) << 8; $CHILD_ERROR = $? >> 8;
$__ps_status0 = $CHILD_ERROR;
do { unlink($__ps_tmp2, $__ps_tmp3); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
$CHILD_ERROR = $__ps_status0;
say "bc -s exit: $CHILD_ERROR";  # line 76
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { unlink('/tmp/cmp_a.txt', '/tmp/cmp_same.txt', '/tmp/cmp_diff.txt', '/tmp/cmp_diff2.txt', '/tmp/cmp_short.txt', '/tmp/cmp_empty.txt'); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };  # line 79

exit $? >> 8;
