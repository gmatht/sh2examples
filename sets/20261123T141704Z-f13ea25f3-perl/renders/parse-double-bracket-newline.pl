#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $var = '';
my $other = '';

if ($var ne "" && !($other ne "")) {
    print "test\n";
    $? = 0 << 8;
}

exit $? >> 8;
