#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $s;
my $i;

$s = "0";  # line 4
$i = "0";
while (int($i) < int(100)) {
    $s = int(($s + $i));
    $i = int(($i + 1));
}
say "sum=$s";  # line 10
$? = 0 << 8;

exit $? >> 8;
