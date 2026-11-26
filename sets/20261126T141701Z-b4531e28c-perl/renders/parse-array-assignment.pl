#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my @arr;
my $arr;

@arr = ("one", "two", "three");  # line 3
say $arr[1];  # line 4
$? = 0 << 8;

exit $? >> 8;
