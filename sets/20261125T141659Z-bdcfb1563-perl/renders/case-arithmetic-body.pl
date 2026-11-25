#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;

use feature 'say';

if (($CHILD_ERROR) =~ /^(?:0)$/) {  # line 3
    say int((1 + 2));
    } else {
    print "other\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    }

exit $? >> 8;
