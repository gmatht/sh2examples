#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


if ((0+($? >> 8)) == (0+'0')) {
    print "success\n";
    $? = 0 << 8;
}

exit $? >> 8;
