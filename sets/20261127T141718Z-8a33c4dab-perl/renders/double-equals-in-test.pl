#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $var = '';

if (!($var ne "value")) {
    print "match\n";
    $? = 0 << 8;
}

exit $? >> 8;
