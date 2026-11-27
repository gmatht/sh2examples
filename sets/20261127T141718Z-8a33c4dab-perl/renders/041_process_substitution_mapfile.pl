#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';
use English qw(-no_match_vars $ERRNO $EVAL_ERROR $INPUT_RECORD_SEPARATOR $OS_ERROR $PROGRAM_NAME);

my $__ps_tmp0;
my @lines;

print "== readarray/mapfile ==\n";
$? = 0 << 8;
$__ps_tmp0 = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
open my $__fh, '>', $__ps_tmp0 or die "Cannot write file: $!\n";
print {$__fh} "x\ny\n";
close $__fh;
$? = 0 << 8;
@lines = do { open(my $__mf, '<', $__ps_tmp0) or die "mapfile: $ERRNO"; my @__l; my $__i = 0; while (my $__line = <$__mf>) { if ($__i < 0) { $__i++; next; } chomp $__line; push @__l, $__line; } close($__mf); @__l }; $? = 0 << 8;
do { unlink($__ps_tmp0); $? = 0 << 8; };
my $__sh2_pf_bad0 = 0;  # line 8
my $__sh2_pf_fl0;
my @__sh2_pa0 = (@lines);
my $__sh2_pa0_n = @__sh2_pa0 || 1;
my @__sh2_pa0_fl = (0);
my @__sh2_pa0_num = (0);
for (my $__sh2_pa0_v = 0; $__sh2_pa0_v < $__sh2_pa0_n; $__sh2_pa0_v++) {
    next unless $__sh2_pa0_num[$__sh2_pa0_v % 1];
    $__sh2_pf_fl0 = $__sh2_pa0_fl[$__sh2_pa0_v % 1];
    $__sh2_pa0[$__sh2_pa0_v] = ($__sh2_pf_fl0 ? (do {
    my ($__pf_s, $__pf_v, $__pf_bad, $__pf_msg) = ($__sh2_pa0[$__sh2_pa0_v], 0, 0, 'invalid number');
    my $__pf_t = $__pf_s;
    $__pf_t =~ s/\A[ \t\n]+//;
    if ($__pf_t =~ /\A([+-]?)0[xX]([0-9a-fA-F]*)(?:\.([0-9a-fA-F]*))?(?:[pP]([+-]?[0-9]+))?\z/s
        && (length($2) || (defined($3) && length($3)))) {
        $__pf_v = hex($2) + ((defined($3) && length($3)) ? hex($3) / 16 ** length($3) : 0);
        $__pf_v *= 2 ** $4 if defined($4) && length($4);
        $__pf_v = -$__pf_v if $1 eq '-';
    } elsif ($__pf_t =~ /\A[+-]?(?:[0-9]+\.?[0-9]*|\.[0-9]+)(?:[eE][+-]?[0-9]+)?\z/s) {
        $__pf_v = 0 + $__pf_t;
    } else {
        if ($__pf_t =~ /\A([+-]?)0[xX]([0-9a-fA-F]*)/) {
            $__pf_v = hex($2);
            $__pf_v = -$__pf_v if $1 eq '-';
            $__pf_msg = 'invalid hex number';
        } elsif ($__pf_t =~ /\A([+-]?(?:[0-9]+\.?[0-9]*|\.[0-9]+)(?:[eE][+-]?[0-9]+)?)/) {
            $__pf_v = 0 + $1;
        }
        $__pf_bad = 1;
    }
    if ($__pf_bad) { print STDERR "printf: $__pf_s: $__pf_msg\n"; $__sh2_pf_bad0 = 1; }
    $__pf_v
}) : (do {
    my ($__pf_s, $__pf_v, $__pf_bad, $__pf_msg) = ($__sh2_pa0[$__sh2_pa0_v], 0, 0, 'invalid number');
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
    if ($__pf_bad) { print STDERR "printf: $__pf_s: $__pf_msg\n"; $__sh2_pf_bad0 = 1; }
    $__pf_v
}));
}
for (my $__sh2_pa0_i = 0; $__sh2_pa0_i < $__sh2_pa0_n; $__sh2_pa0_i += 1) {
    my $__sh2_pa0_have = ($__sh2_pa0_i + 1 > $__sh2_pa0_n) ? $__sh2_pa0_n - $__sh2_pa0_i : 1;
    my @__sh2_pa0_pad = $__sh2_pa0_have >= 1 ? () : map { $__sh2_pa0_num[$_] ? 0 : '' } ($__sh2_pa0_have .. 1 - 1);
    printf("%s ", @__sh2_pa0[$__sh2_pa0_i .. ($__sh2_pa0_i + 1 - 1 < $__sh2_pa0_n ? $__sh2_pa0_i + 1 - 1 : $__sh2_pa0_n - 1)], @__sh2_pa0_pad);
}
$? = (($__sh2_pf_bad0 ? 1 : 0)) << 8;
print "\n";  # line 8
$? = 0 << 8;

exit $? >> 8;
