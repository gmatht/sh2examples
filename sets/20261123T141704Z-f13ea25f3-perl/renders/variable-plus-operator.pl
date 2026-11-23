#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


if (length((defined($ENV{MYVAR}) ? "set" : '')) ne 0) {
    print "MYVAR is set\n";
    $? = 0 << 8;
}

exit $? >> 8;
