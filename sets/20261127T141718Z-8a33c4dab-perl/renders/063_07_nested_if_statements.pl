#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;
my $var = '';
my $file = '';
my @array;

if (($var =~ /^[0-9]+$/) && ($var > 0) && -f $file) {
    if ((join((defined($ENV{IFS}) && length($ENV{IFS})) ? substr($ENV{IFS}, 0, 1) : ' ', @array) =~ /value/) || (scalar(@array) > 5)) {
        $CHILD_ERROR = (length("$(echo")) ? 0 : 1;
        if (length("$(echo")) {
            print "Deeply nested condition met\n";
            $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        } else {
            $CHILD_ERROR = 0;
        }
    }
}

exit $? >> 8;
