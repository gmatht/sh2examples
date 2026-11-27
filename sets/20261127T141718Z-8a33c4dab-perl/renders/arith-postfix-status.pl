#!/usr/bin/env perl
use strict;
use warnings;
my $i;

$i = "0";  # line 5
$? = (((($i++)) != 0) ? 0 : 1) << 8;
print("post0=" . (($? >> 8)), "\n");  # line 7
$? = 0 << 8;
$? = ((((++$i)) != 0) ? 0 : 1) << 8;
print("pre=" . (($? >> 8)), "\n");  # line 9
$? = 0 << 8;
$i = "5";  # line 10
$? = (((($i++)) != 0) ? 0 : 1) << 8;
print("post5=" . (($? >> 8)), "\n");  # line 12
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
