#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


if (!($ENV{VAR1} ne "") && !($ENV{VAR2} ne "")) {
    print "Both empty\n";
    $? = 0 << 8;
}

exit $? >> 8;
