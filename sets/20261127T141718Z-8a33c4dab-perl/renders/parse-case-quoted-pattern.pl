#!/usr/bin/env perl
use strict;
use warnings;
my $i = '';

$? = 0 << 8;  # line 5
if (($i) =~ /^(?:.*'
'.*|.*'&'.*|.*''.*|.*'\|'.*)$/) {
    print "match\n";
    $? = 0 << 8;
    $? = 0 << 8;
}

exit $? >> 8;
