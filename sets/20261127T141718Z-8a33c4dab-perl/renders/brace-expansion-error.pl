#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "1 2 3 4 5 6 7 8 9 10\n";
$? = 0 << 8;
print "a b c\n";
$? = 0 << 8;
print "file.txt file.md\n";
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 7
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
