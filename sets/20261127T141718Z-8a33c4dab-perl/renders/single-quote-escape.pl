#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "it's works\n";  # line 2
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 4
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
