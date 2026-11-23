#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;
my $__nocasematch = 0;

if ((($__nocasematch) ? lc($ENV{HOME}) eq lc($ENV{HOME}) : $ENV{HOME} eq $ENV{HOME})) {  # line 1
    print "1\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } else {
    print "-\n";
    }
if ((($__nocasematch) ? lc("$ENV{HOME}/Documents") eq lc($ENV{HOME}) : "$ENV{HOME}/Documents" eq $ENV{HOME})) {  # line 2
    print "2\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } else {
    print "-\n";
    }
$CHILD_ERROR = ((($__nocasematch) ? lc("$ENV{HOME}/Documents") eq lc("$ENV{HOME}/Documents") : "$ENV{HOME}/Documents" eq "$ENV{HOME}/Documents")) ? 0 : 1;  # line 3
if ((($__nocasematch) ? lc("$ENV{HOME}/Documents") eq lc("$ENV{HOME}/Documents") : "$ENV{HOME}/Documents" eq "$ENV{HOME}/Documents")) {
    print "3\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    print "-\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}

exit $? >> 8;
