#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $i;
my $j;

$i = int(2);
say "i=$i";  # line 5
$? = 0 << 8;
$j = (($i++) + (++$i));  # line 6
say "j=$j";  # line 7
$? = 0 << 8;

exit $? >> 8;
