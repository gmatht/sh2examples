#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "Sort test:\n";
$? = 0 << 8;
do { my @__pl = split(/\n/, q{c
b
a}); pop @__pl if @__pl && $__pl[$#__pl] eq ''; @__pl = sort { our ($a, $b); $a cmp $b } @__pl; say join("\n", @__pl);  };
print "---\n";
$? = 0 << 8;
do { my @__pl = split(/\n/, q{3
1
2}); pop @__pl if @__pl && $__pl[$#__pl] eq ''; @__pl = sort { our ($a, $b); $a <=> $b } @__pl; say join("\n", @__pl); $? = 0 << 8; };
print "done\n";
$? = 0 << 8;

exit $? >> 8;
