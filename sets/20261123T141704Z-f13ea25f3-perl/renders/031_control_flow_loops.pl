#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $i;
my $j;

for my $__i (("1", "2", "3", "4", "5")) {
    $i = $__i;
    say "Number: $i";
    $? = 0 << 8;
}
for my $__i (("1", "2", "3")) {
    $i = $__i;
    $j = int(($j + 1));
}
say join(' ', grep { defined($_) && length($_) } split(/\s+/, $j));  # line 8
$i = "0";
while (int($i) < int(10)) {
    say "Counter: $i";
    $? = 0 << 8;
    $i = int(($i + 1));
}

exit $? >> 8;
