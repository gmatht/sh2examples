#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;
my $__nocasematch = 0;
my $__sh2_tc0;

$__sh2_tc0 = "hello";
$CHILD_ERROR = ((($__nocasematch) ? lc($__sh2_tc0) eq lc('hello') : $__sh2_tc0 eq 'hello')) ? 0 : 1;
if ((($__nocasematch) ? lc($__sh2_tc0) eq lc('hello') : $__sh2_tc0 eq 'hello')) {
    print "match\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    $CHILD_ERROR = 0;
}

exit $? >> 8;
