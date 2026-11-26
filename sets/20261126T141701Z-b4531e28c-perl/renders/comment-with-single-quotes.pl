#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;

use feature 'say';

print("parsed single-quotes in comments OK\n");  # line 4
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "done: $CHILD_ERROR";  # line 6
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
