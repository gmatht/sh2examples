#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $i;
my $j;

$i = int(2);
$j = (($i++) + (++$i));  # line 5
say "i=$i, j=$j";  # line 6
$? = 0 << 8;

exit $? >> 8;
