#!/usr/bin/env perl
use strict;
use warnings;
my $x;
my $a = '';
my $b = '';
my $c = '';
my $d = '';

$x = (do { my $__a = (do { my $__d = ($c + $d); eval { int((($a + $b)) / ($__d)) } }); defined($__a) ? int($__a) : undef });  # line 4
print "x=[$x]\n";
$? = 0 << 8;

exit $? >> 8;
