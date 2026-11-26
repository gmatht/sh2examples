#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $i;

print "1 2 3 4 5\n";
$? = 0 << 8;
print "file.txt file.bak\n";
$? = 0 << 8;
for my $__i (("1", "2", "3")) {
    $i = $__i;
    say $i;
    $? = 0 << 8;
}

exit $? >> 8;
