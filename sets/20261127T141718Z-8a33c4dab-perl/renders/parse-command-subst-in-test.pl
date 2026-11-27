#!/usr/bin/env perl
use strict;
use warnings;
my $__sh2_tc0;

$__sh2_tc0 = "foo";
if (int($__sh2_tc0) ne int(0)) {
    print "bar\n";
    $? = 0 << 8;
}

exit $? >> 8;
