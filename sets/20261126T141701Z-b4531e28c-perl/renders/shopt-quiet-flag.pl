#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;

use feature 'say';

$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 3
$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 4
print("parsed OK\n");
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "done: $CHILD_ERROR";  # line 7
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
