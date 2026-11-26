#!/usr/bin/env perl
use strict;
use warnings;
my $__sh2_tc0;

$__sh2_tc0 = "foo";
if ($__sh2_tc0 ne "") {
    print "yes\n";
    $? = 0 << 8;
}

exit $? >> 8;
