#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;
my $__nocasematch = 0;

$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 4
$__nocasematch = 1;$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 5
print "Extended glob patterns enabled\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "exit: $CHILD_ERROR";  # line 9
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
