#!/usr/bin/env perl
use strict;
use warnings;
my $__sh2_tc0;

$__sh2_tc0 = "hello";
if (length($__sh2_tc0) ne 0) {
    print "non-empty\n";
    $? = 0 << 8;
}

exit $? >> 8;
