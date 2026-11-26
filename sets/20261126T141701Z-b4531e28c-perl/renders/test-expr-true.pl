#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $var = '';

if (!($var ne "true")) {
    print "it's true\n";
    $? = 0 << 8;
}

exit $? >> 8;
