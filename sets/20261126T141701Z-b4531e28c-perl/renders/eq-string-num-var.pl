#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;
my $__nocasematch = 0;
my $i;

for my $__i (("1", "2", "3")) {
    $i = $__i;
$? = ((!($i ne "2")) ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;
if (!($i ne "2")) {
    say "hit=$i";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
}
if ((($__nocasematch) ? lc($CHILD_ERROR) eq lc('0') : $CHILD_ERROR eq '0')) {
    print "status-zero\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    }
$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 8
$CHILD_ERROR = ((($__nocasematch) ? lc($CHILD_ERROR) eq lc('0') : $CHILD_ERROR eq '0')) ? 0 : 1;
if ((($__nocasematch) ? lc($CHILD_ERROR) eq lc('0') : $CHILD_ERROR eq '0')) {
    print "status-zero2\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    $CHILD_ERROR = 0;
}

exit $? >> 8;
