#!/usr/bin/env perl
use strict;
use warnings;

$? = 0 << 8;  # line 3
if (("word") =~ /^(?:\$\(echo "pattern"\))$/) {
    print "matched\n";
    $? = 0 << 8;
    $? = 0 << 8;
}

exit $? >> 8;
