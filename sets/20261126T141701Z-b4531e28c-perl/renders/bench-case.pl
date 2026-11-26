#!/usr/bin/env perl
use strict;
use warnings;

if (("hello") =~ /^(?:.*llo)$/) {
    print "case=match\n";
    $? = 0 << 8;
    } else {
    print "case=no\n";
    $? = 0 << 8;
    }
if (("abc") =~ /^(?:.*llo)$/) {
    print "case2=match\n";
    $? = 0 << 8;
    } else {
    print "case2=no\n";
    $? = 0 << 8;
    }

exit $? >> 8;
