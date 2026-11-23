#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $i;
my $__n;

$i = "1";  # line 3
$__n = "0";
while (int($__n) < int(1000)) {
    $? = (((($i++)) != 0) ? 0 : 1) << 8;
    $__n = int(($__n + 1));
}
say "increment=$i";  # line 9
$? = 0 << 8;

exit $? >> 8;
