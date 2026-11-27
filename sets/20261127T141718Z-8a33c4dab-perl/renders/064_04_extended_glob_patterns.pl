#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $__nocasematch = 0;

$? = 0 << 8;  # line 4
$__nocasematch = 1;$? = 0 << 8;  # line 5
print "Extended glob patterns enabled\n";
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 9
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
