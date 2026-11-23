#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

if ((0+$CHILD_ERROR) == (0+'0')) {
    print "success\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}

exit $? >> 8;
