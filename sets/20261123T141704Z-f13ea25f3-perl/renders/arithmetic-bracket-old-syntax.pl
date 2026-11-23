#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $i;
my $j;

$i = "0";  # line 3
$i = ($i + 1);  # line 4
$j = ($j + 1);  # line 5
say join(' ', grep { defined($_) && length($_) } split(/\s+/, $i), split(/\s+/, $j));  # line 6
$? = 0 << 8;

exit $? >> 8;
