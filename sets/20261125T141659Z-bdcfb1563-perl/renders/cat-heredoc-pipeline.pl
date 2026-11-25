#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;

use feature 'say';

print '1', "\n"; $? = 0 << 8; $CHILD_ERROR = $? >> 8;say "exit: $CHILD_ERROR";  # line 9
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
