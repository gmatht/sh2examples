#!/usr/bin/env perl
use strict;
use warnings;

$? = (('foo' eq 'bar') ? 0 : 1) << 8;
do { my $__x = 'a=b'; $__x =~ s{=}{:}; print STDOUT $__x, "\n"; }; $? = 0 << 8;print("exit: " . (($? >> 8)), "\n");  # line 7
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
