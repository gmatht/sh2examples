#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

if (-S '/dev/null') {
    print "is a socket\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    }
if (-S '/dev/null') {
    print "is a socket\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    }
$CHILD_ERROR = (-S '/dev/null') ? 0 : 1;
if (-S '/dev/null') {
    print "is_socket: yes\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    print "is_socket: no\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
say "done: $CHILD_ERROR";  # line 13
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
