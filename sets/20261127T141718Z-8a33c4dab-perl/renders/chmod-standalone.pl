#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "chmod 600 myfile\n";
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 6
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
