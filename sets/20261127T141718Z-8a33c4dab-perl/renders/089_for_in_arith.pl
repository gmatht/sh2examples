#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $total;
my $i;

$total = "0";  # line 4
my $__sh_loop_rc0 = 0;  # line 5
for my $__i (("1", "2", "3", "4", "5")) {
    $i = $__i;
    $total = int(($total + $i));
    $__sh_loop_rc0 = $? >> 8;
}
$? = ($__sh_loop_rc0) << 8;
say "total=$total";  # line 8
$? = 0 << 8;

exit $? >> 8;
