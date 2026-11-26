#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $s;
my $i;
my $ch;

$s = "HELLOWORLD";
$i = "0";  # line 11
while (int($i) < int(10)) {
    $ch = substr($s, $i, 1);
    print($ch);
    $? = 0 << 8;
    $i = int(($i + 1));
}
print "\n";
$? = 0 << 8;

exit $? >> 8;
