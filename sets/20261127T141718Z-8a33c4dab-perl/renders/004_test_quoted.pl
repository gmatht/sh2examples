#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "Hello, World!\n";
$? = 0 << 8;
print "Single quoted\n";  # line 2
$? = 0 << 8;
print "String with \"escaped\" quotes\n";
$? = 0 << 8;
print "String with 'single' quotes\n";
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 6
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
