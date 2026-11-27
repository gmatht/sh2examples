#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $__nocasematch = 0;
my $never_set = '';
my $never_set2 = '';

if ((($__nocasematch) ? lc($never_set) eq lc('') : $never_set eq '')) {  # line 4
    print "unset-is-empty\n";
    $? = 0 << 8;
} else {
    print "unset-not-empty\n";
    $? = 0 << 8;
}
if (!($never_set2 ne "")) {
    print "zero-len\n";
    $? = 0 << 8;
}

exit $? >> 8;
