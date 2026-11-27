#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "hello world\r\n";  # line 2
$? = 0 << 8;
print "goodbye\r\n";  # line 3
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 5
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
