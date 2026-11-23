#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;
use feature 'say';

my $i;

$i = "0";  # line 5
$? = (((($i++)) != 0) ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;
say "post0=$CHILD_ERROR";  # line 7
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = ((((++$i)) != 0) ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;
say "pre=$CHILD_ERROR";  # line 9
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$i = "5";  # line 10
$? = (((($i++)) != 0) ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;
say "post5=$CHILD_ERROR";  # line 12
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
