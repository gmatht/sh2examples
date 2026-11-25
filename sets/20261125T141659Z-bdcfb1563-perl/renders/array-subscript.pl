#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my @arr;
my $arr;

@arr = ("a", "b", "c");  # line 2
say $arr[1];  # line 3
$? = 0 << 8;

exit $? >> 8;
