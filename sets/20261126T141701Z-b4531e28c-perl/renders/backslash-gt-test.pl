#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

$CHILD_ERROR = (('5' gt '10')) ? 0 : 1;
if (('5' gt '10')) {
    print "a > b\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    $CHILD_ERROR = 0;
}
print("a=[5]\n");
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print("b=[10]\n");
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
