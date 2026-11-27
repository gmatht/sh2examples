#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


$? = 0 << 8;  # line 4
print "== Advanced brace expansion ==\n";
$? = 0 << 8;
print "a1 a2 a3 b1 b2 b3 c1 c2 c3\n";
$? = 0 << 8;
print "1 3 5 7 9\n";
$? = 0 << 8;
print "a d g j m p s v y\n";
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 11
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
