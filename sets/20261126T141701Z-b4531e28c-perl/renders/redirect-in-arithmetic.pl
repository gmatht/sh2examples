#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my @arr;
my $arr;

@arr = ("10", "20", "30");  # line 4
say int((${arr}[1] > 2));  # line 5
$? = 0 << 8;

exit $? >> 8;
