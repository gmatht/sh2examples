#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $i;

print "Hello, World!\n";
$? = 0 << 8;
if (-f 'test.txt') {
    print "File exists\n";
    $? = 0 << 8;
} else {
    }
for my $__i (("1", "2", "3", "4", "5")) {
    $i = $__i;
    say join(' ', grep { defined($_) && length($_) } split(/\s+/, $i));
    $? = 0 << 8;
}

exit $? >> 8;
