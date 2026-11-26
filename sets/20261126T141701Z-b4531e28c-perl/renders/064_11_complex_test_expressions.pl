#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


if (('42' =~ /^[0-9]+$/) && ('debug' eq 'test' || 'debug' eq 'debug')) {
    print "Valid input\n";
    $? = 0 << 8;
}

exit $? >> 8;
