#!/usr/bin/env perl
use strict;
use warnings;

if (("hello") =~ /^(?:hello)$/) {
    print "Hi!\n";
    $? = 0 << 8;
    } elsif (("hello") =~ /^(?:bye)$/) {
    print "Bye!\n";
    $? = 0 << 8;
    } else {
    print "Other\n";
    $? = 0 << 8;
    }

exit $? >> 8;
