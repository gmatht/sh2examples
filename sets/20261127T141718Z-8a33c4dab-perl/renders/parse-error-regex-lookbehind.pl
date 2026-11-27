#!/usr/bin/env perl
use strict;
use warnings;

do { my $__t = 'hello' . "\n"; $__t =~ tr/h/H/; print $__t; $? = 0 << 8; };
print("exit: " . (($? >> 8)), "\n");  # line 5
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
