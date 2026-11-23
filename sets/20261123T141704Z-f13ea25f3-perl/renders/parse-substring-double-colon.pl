#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $x;

$x = "hello world";
say join(' ', substr($x, 0, -2));  # line 4
say join(' ', substr($x, -3));  # line 5
$? = 0 << 8;

exit $? >> 8;
