#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

while (!(1)) {
    print "waiting\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
say "exit: $CHILD_ERROR";  # line 7
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
