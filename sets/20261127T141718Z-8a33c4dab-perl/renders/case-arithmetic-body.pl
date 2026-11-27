#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;

use feature 'say';

$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 3
if (($CHILD_ERROR) =~ /^(?:0)$/) {
    say int((1 + 2));
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    print "other\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}

exit $? >> 8;
