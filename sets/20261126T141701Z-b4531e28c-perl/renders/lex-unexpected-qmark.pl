#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;
my $var = '';

$CHILD_ERROR = (($var =~ /^(?s)..*$/)) ? 0 : 1;
if (($var =~ /^(?s)..*$/)) {
    print "has chars\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    $CHILD_ERROR = 0;
}

exit $? >> 8;
