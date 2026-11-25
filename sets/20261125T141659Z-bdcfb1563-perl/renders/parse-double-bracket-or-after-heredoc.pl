#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

if (-d '/path1' || -d '/path2') {
    print "found\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
say "done: $CHILD_ERROR";  # line 12
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
