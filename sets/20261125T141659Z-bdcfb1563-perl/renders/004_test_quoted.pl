#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

print "Hello, World!\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "Single quoted\n";  # line 2
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "String with \"escaped\" quotes\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "String with 'single' quotes\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "exit: $CHILD_ERROR";  # line 6
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
