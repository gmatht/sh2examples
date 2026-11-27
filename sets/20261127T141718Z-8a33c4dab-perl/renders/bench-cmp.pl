#!/usr/bin/env perl
use strict;
use warnings;
my $__nocasematch = 0;

$? = (((($__nocasematch) ? lc('abc') eq lc('abc') : 'abc' eq 'abc')) ? 0 : 1) << 8;  # line 3
print("eq=" . (($? >> 8)), "\n");  # line 3
$? = 0 << 8;
$? = (((!((($__nocasematch) ? lc('abc') eq lc('xyz') : 'abc' eq 'xyz')))) ? 0 : 1) << 8;  # line 4
print("ne=" . (($? >> 8)), "\n");  # line 4
$? = 0 << 8;
$? = (('5' < '10') ? 0 : 1) << 8;  # line 5
print("lt=" . (($? >> 8)), "\n");  # line 5
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
