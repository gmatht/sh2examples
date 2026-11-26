#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $i;

$i = "0";
while (int($i) < int(3)) {
    say "i=$i";
    $? = 0 << 8;
    $i = int(($i + 1));
}

exit $? >> 8;
