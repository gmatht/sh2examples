#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;

use feature 'say';

while (1) {
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    last;
    if ($CHILD_ERROR != 0) {
        exit "1";
    }
}
print("parsed OK\n");
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "exit: $CHILD_ERROR";  # line 5
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
