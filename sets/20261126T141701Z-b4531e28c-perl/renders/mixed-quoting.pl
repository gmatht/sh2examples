#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $x;
my $z;

$x = ($ENV{HOME}) . "/file";  # line 3
$z = ($x) . " with " . ("literal string") . " and " . ($x) . "ext";
say $z;  # line 6
$? = 0 << 8;
print("y=[literal string]\n");
$? = 0 << 8;
print "x=[$x]\n";
$? = 0 << 8;

exit $? >> 8;
