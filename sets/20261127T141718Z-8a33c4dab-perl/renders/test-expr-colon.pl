#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;
my $line = '';

$CHILD_ERROR = (($line =~ 'DEBUG:')) ? 0 : 1;
if (($line =~ 'DEBUG:')) {
    print "match\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    $CHILD_ERROR = 0;
}

exit $? >> 8;
