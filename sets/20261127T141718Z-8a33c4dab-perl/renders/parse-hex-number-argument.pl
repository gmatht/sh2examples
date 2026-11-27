#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "0x1234\n";  # line 2
$? = 0 << 8;
print("hex_val: " . (255), "\n");  # line 3
$? = 0 << 8;
print("done: " . (($? >> 8)), "\n");  # line 4
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
