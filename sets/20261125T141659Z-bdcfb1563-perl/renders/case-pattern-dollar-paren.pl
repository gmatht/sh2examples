#!/usr/bin/env perl
use strict;
use warnings;
my $prev = '';

if (($prev) =~ /^(?:\$\(echo "foo\|bar"\))$/) {  # line 4
    print "matched\n";
    $? = 0 << 8;
    }

exit $? >> 8;
