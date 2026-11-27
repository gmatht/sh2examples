#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;
my $release = '';

$CHILD_ERROR = (($release =~ 'bullseye|bookworm|jammy')) ? 0 : 1;
if (($release =~ 'bullseye|bookworm|jammy')) {
    print "match\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    $CHILD_ERROR = 0;
}

exit $? >> 8;
