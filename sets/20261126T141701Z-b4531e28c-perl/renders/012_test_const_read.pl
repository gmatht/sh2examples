#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $i;

$i = "0";
while (int($i) < int(10)) {
    print($i);
    $? = 0 << 8;
    $i = int(($i + 1));
}
print "\n";
$? = 0 << 8;

exit $? >> 8;
