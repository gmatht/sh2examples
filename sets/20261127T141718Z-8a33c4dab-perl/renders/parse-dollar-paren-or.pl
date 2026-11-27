#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $x;
my $r = '';

$r = "hello";
if (($? >> 8) != 0) {
    $x = ($? >> 8);
}
say $r;  # line 10
$? = 0 << 8;
print "x=[$x]\n";
$? = 0 << 8;

exit $? >> 8;
