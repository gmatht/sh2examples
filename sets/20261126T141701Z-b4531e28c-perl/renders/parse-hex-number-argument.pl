#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

print "0x1234\n";  # line 2
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print("hex_val: " . (255), "\n");  # line 3
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "done: $CHILD_ERROR";  # line 4
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
