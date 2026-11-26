#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;

use feature 'say';

$ENV{LANG} = "C";  # line 5
$? = (('a' eq 'a') ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;
print("parsed OK\n");
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "done: $CHILD_ERROR";  # line 9
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
