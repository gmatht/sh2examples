#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;

use feature 'say';

say join(' ', grep { defined($_) && length($_) } split(/\s+/, $CHILD_ERROR));  # line 4
say join(' ', @ARGV);  # line 5
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say join(' ', @ARGV);  # line 6
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
