#!/usr/bin/env perl
use strict;
use warnings;
my $i = '';

if (($i) =~ /^(?:.*'
'.*|.*'&'.*|.*''.*|.*'\|'.*)$/) {  # line 5
    print "match\n";
    $? = 0 << 8;
    }

exit $? >> 8;
