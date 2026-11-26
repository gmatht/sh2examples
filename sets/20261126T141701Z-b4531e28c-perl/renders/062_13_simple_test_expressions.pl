#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

print "Testing simple test expressions...\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$CHILD_ERROR = (-f 'file.txt') ? 0 : 1;
if (-f 'file.txt') {
    print "File exists\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    print "File does not exist\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
say "done: $CHILD_ERROR";  # line 11
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
