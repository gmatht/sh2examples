#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;
use feature 'say';

my $__nocasematch = 0;

$? = (((($__nocasematch) ? lc('abc') eq lc('abc') : 'abc' eq 'abc')) ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;  # line 3
say "eq=$CHILD_ERROR";  # line 3
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = (((!((($__nocasematch) ? lc('abc') eq lc('xyz') : 'abc' eq 'xyz')))) ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;  # line 4
say "ne=$CHILD_ERROR";  # line 4
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = (('5' < '10') ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;  # line 5
say "lt=$CHILD_ERROR";  # line 5
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
