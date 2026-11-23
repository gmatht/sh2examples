#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;
my $__nocasematch = 0;

if ((($__nocasematch) ? lc('a') eq lc('a') : 'a' eq 'a')) {  # line 5
    print "eq\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    }
if ((!(($__nocasematch) ? lc('a') eq lc('a') : 'a' eq 'a'))) {  # line 6
    print "bad\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    print "ne\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
if ((($__nocasematch) ? lc('a') eq lc('b') : 'a' eq 'b')) {  # line 7
    print "bad\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    print "not-equal\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
if ((($__nocasematch) ? lc('hello') eq lc('hello') : 'hello' eq 'hello')) {
    print "var-eq\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    }
if ((!(($__nocasematch) ? lc('hello') eq lc('other') : 'hello' eq 'other'))) {
    print "var-ne\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    }
if ((($__nocasematch) ? lc('10') eq lc('10') : '10' eq '10')) {  # line 13
    print "num-eq\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    }
$CHILD_ERROR = ((($__nocasematch) ? lc('10') eq lc('010') : '10' eq '010')) ? 0 : 1;  # line 14
if ((($__nocasematch) ? lc('10') eq lc('010') : '10' eq '010')) {
    print "bad\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    print "num-string-distinct\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}

exit $? >> 8;
