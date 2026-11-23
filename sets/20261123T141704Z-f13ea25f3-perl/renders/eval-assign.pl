#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $x;
my $y;

$x = 5;  # line 3
$y = ($x) . "+1";  # line 4
say "y=$y";  # line 5
$? = 0 << 8;

exit $? >> 8;
