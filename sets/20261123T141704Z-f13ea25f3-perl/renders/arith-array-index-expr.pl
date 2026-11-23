#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my @arr;
my $i;
my $arr;

@arr = ("10", "20", "30");  # line 3
$i = 1;  # line 4
say int((${arr}[$i] * 2));  # line 5
$? = 0 << 8;
say int((${arr}[0] + ${arr}[2]));  # line 6
$? = 0 << 8;
say int(((0+(${arr}[1] + ${arr}[2])) & (0+50)));  # line 7
$? = 0 << 8;

exit $? >> 8;
