#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $i;
my $__n;

$i = "1";  # line 3
$__n = "0";
my $__sh_loop_rc0 = 0;
while (int($__n) < int(1000)) {
    $i = int(($i + 1));
    $__n = int(($__n + 1));
    $__sh_loop_rc0 = $? >> 8;
}
$? = ($__sh_loop_rc0) << 8;
say "count=$i";  # line 9
$? = 0 << 8;

exit $? >> 8;
