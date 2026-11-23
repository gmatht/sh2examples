#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $i;

for my $__i ((@ARGV)) {  # line 2
    $i = $__i;
    say $i;
    $? = 0 << 8;
}
for my $__i (join(' ', @ARGV)) {  # line 3
    $i = $__i;
    say $i;
    $? = 0 << 8;
}

exit $? >> 8;
