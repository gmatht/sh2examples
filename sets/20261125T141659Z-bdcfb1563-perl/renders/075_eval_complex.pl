#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $x;

$x = 42;  # line 4
say "The answer is $x";  # line 5
$? = 0 << 8;

exit $? >> 8;
