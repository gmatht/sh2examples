#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


$? = 0 << 8;  # line 4
print "== Basic brace expansion ==\n";
$? = 0 << 8;
print "1 2 3 4 5\n";
$? = 0 << 8;
print "a b c\n";
$? = 0 << 8;
print "00 02 04\n";
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 11
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
