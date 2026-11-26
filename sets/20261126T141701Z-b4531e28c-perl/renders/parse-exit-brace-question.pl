#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;

exit $CHILD_ERROR;  # line 3
print("parsed OK\n");
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
