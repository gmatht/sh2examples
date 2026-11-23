#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;

use feature 'say';

do { my @__h = split(/\n/, 'select', -1); pop @__h if @__h && $__h[-1] eq q{}; splice(@__h, 1) if @__h > 1; if (@__h) { print STDOUT join("\n", @__h), "\n"; } }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;say "exit: $CHILD_ERROR";  # line 6
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
