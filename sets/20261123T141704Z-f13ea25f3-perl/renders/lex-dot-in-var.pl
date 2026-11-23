#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


if (1) {  # line 3
    print "failed\n";
    $? = 0 << 8;
}

exit $? >> 8;
