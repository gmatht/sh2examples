#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $count;

$count = "3";
while (!(int($count) == int(0))) {
    say "count=$count";
    $? = 0 << 8;
    $count = int(($count - 1));
}

exit $? >> 8;
