#!/usr/bin/env perl
use strict;
use warnings;
my $needop = '';

if (($needop) =~ /^(?:)$/) {
    print "empty\n";
    $? = 0 << 8;
    } else {
}

exit $? >> 8;
