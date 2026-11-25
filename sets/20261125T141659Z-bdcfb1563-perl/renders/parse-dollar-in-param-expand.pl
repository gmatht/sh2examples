#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $x;
my $y = '';

$x = ($y) . "extra";  # line 2
say $x;  # line 3
$? = 0 << 8;

exit $? >> 8;
