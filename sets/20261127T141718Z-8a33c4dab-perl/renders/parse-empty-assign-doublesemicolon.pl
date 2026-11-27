#!/usr/bin/env perl
use strict;
use warnings;
my $needop = '';

$? = 0 << 8;
if (($needop) =~ /^(?:)$/) {
    print "empty\n";
    $? = 0 << 8;
    $? = 0 << 8;
} else {
}

exit $? >> 8;
