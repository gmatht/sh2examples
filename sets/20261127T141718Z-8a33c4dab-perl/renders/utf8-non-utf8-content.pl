#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "Test non-UTF-8:\n";
$? = 0 << 8;
print "acc\xE9nt\n";
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 6
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
