#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "Testing ambiguous operators...\n";
$? = 0 << 8;
print("2**3**2 = " . (512), "\n");
$? = 0 << 8;

exit $? >> 8;
