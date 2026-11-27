#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


$? = 0 << 8;  # line 4
if ((0+($? >> 8)) == (0+'0')) {
    print "ok\n";
    $? = 0 << 8;
}

exit $? >> 8;
