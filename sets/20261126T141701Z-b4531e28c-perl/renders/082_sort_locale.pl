#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;

use feature 'say';

do { my @__pl = split(/\n/, q{c
a
b}); pop @__pl if @__pl && $__pl[$#__pl] eq ''; @__pl = sort { our ($a, $b); $a cmp $b } @__pl; say join("\n", @__pl); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
say "exit: $CHILD_ERROR";  # line 6
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
