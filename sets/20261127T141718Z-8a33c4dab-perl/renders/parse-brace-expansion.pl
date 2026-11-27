#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "1 2 3 4 5\n";
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 5
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
