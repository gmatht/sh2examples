#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

$CHILD_ERROR = (-f $ENV{CONF}) ? 0 : 1;
if (-f $ENV{CONF}) {
    print "exists\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    $CHILD_ERROR = 0;
}

exit $? >> 8;
