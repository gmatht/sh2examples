#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "1 2 3 4 5\n";
$? = 0 << 8;
print "a b c d e\n";
$? = 0 << 8;
print "1..3 7..9\n";
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 8
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
