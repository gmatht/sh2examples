#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $var = '';
my $file = '';
my $dir = '';

if (((length($var) && -f $file) || -d $dir) && length("$(wc -l <")) {
    print "Complex test passed\n";
    $? = 0 << 8;
}

exit $? >> 8;
