#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


if ((0+scalar(@ARGV)) < (0+'2')) {
    print "One\n";
    $? = 0 << 8;
    print "Two\n";
    $? = 0 << 8;
}

exit $? >> 8;
