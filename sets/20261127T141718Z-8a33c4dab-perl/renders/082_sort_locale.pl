#!/usr/bin/env perl
use strict;
use warnings;

use feature 'say';

do { my @__pl = split(/\n/, q{c
a
b}); pop @__pl if @__pl && $__pl[$#__pl] eq ''; @__pl = sort { our ($a, $b); $a cmp $b } @__pl; say join("\n", @__pl); $? = 0 << 8; };
print("exit: " . (($? >> 8)), "\n");  # line 6
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
