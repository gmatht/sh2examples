#!/usr/bin/env perl
use strict;
use warnings;

if (("word") =~ /^(?:\$\(echo "pattern"\))$/) {  # line 3
    print "matched\n";
    $? = 0 << 8;
    }

exit $? >> 8;
