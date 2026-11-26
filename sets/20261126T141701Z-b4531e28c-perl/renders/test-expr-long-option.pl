#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

$CHILD_ERROR = ((join(' ', @ARGV) =~ /^(?s).*--view.*$/)) ? 0 : 1;
if ((join(' ', @ARGV) =~ /^(?s).*--view.*$/)) {
    print "view mode\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    $CHILD_ERROR = 0;
}

exit $? >> 8;
