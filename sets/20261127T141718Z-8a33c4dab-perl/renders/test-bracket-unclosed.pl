#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $file = '';

if (-f $file && -r $file) {  # line 3
    print "readable\n";
    $? = 0 << 8;
}

exit $? >> 8;
