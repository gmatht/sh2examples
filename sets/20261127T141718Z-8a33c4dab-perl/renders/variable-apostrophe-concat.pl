#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $y;

$y = ("hello") . "world";
say join(' ', grep { defined($_) && length($_) } split(/\s+/, $y));  # line 5
$? = 0 << 8;
print("x=[hello]\n");
$? = 0 << 8;

exit $? >> 8;
