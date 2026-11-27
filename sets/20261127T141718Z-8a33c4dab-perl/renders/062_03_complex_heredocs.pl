#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "Testing complex here-documents...\n";
$? = 0 << 8;
print('This is a test line
This is not a test line
This is another test line
');
$? = 0 << 8;
print("done: " . (($? >> 8)), "\n");  # line 11
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
