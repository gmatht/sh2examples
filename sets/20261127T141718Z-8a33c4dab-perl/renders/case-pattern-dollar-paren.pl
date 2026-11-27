#!/usr/bin/env perl
use strict;
use warnings;
my $prev = '';

$? = 0 << 8;  # line 4
if (($prev) =~ /^(?:\$\(echo "foo\|bar"\))$/) {
    print "matched\n";
    $? = 0 << 8;
    $? = 0 << 8;
}

exit $? >> 8;
