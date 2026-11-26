#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

print "1 2 3 4 5\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "exit: $CHILD_ERROR";  # line 5
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
