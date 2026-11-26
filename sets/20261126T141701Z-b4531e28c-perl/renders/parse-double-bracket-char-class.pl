#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;
my $x = '';

$CHILD_ERROR = (($x =~ '[abc]')) ? 0 : 1;  # line 4
if (($x =~ '[abc]')) {
    print "match\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    $CHILD_ERROR = 0;
}

exit $? >> 8;
