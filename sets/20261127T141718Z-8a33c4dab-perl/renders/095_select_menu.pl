#!/usr/bin/env perl
use strict;
use warnings;

do { my @__h = split(/\n/, 'select', -1); pop @__h if @__h && $__h[-1] eq q{}; splice(@__h, 1) if @__h > 1; if (@__h) { print STDOUT join("\n", @__h), "\n"; } }; $? = 0 << 8;print("exit: " . (($? >> 8)), "\n");  # line 6
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
