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
my @__sh2_pa0 = (@lines);  # line 8
my $__sh2_pa0_n = @__sh2_pa0 || 1;
for (my $__sh2_pa0_i = 0; $__sh2_pa0_i < $__sh2_pa0_n; $__sh2_pa0_i += 1) {
    printf("%s ", @__sh2_pa0[$__sh2_pa0_i .. ($__sh2_pa0_i + 1 - 1 < $__sh2_pa0_n ? $__sh2_pa0_i + 1 - 1 : $__sh2_pa0_n - 1)], ("") x (($__sh2_pa0_i + 1 > $__sh2_pa0_n) ? $__sh2_pa0_i + 1 - $__sh2_pa0_n : 0));
}
$? = 0 << 8;
print "\n";  # line 8
$? = 0 << 8;

exit $? >> 8;
