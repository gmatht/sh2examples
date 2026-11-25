#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $j;
my $i;

$j = "1";  # line 1
for my $__i (("1", "2", "3", "4", "5")) {
    $i = $__i;
    $j = ($j * $i);
}
say join(' ', grep { defined($_) && length($_) } split(/\s+/, $j));  # line 6
$? = 0 << 8;

exit $? >> 8;
