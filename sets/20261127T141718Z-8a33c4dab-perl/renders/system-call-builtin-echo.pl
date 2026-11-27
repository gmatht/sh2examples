#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "hello world\n";
$? = 0 << 8;
print("done: " . (($? >> 8)), "\n");  # line 7
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
