#!/usr/bin/env perl
use strict;
use warnings;
my $x;
my $n = '';

$x = (do { my $__d = 3; eval { int((($n + 1))) % ($__d) } });  # line 2
print "x=[$x]\n";
$? = 0 << 8;

exit $? >> 8;
