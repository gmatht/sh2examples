#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "Multi-stage:\n";
$? = 0 << 8;
do { my $__x = 'hello world'; $__x =~ tr/ /\n/; my @__l = split(/\n/, $__x, -1); pop @__l if @__l && $__l[-1] eq q{}; @__l = sort @__l; splice(@__l, 2) if @__l > 2; if (@__l) { print STDOUT join("\n", @__l), "\n"; } }; $? = 0 << 8;print "done\n";
$? = 0 << 8;

exit $? >> 8;
