#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $i;

for my $__i (("1", "2", "3")) {
    $i = $__i;
    do { my $__sh_ok = (!($i ne "2")); $? = ($__sh_ok ? 0 : 1) << 8; if (!$__sh_ok) { $? = 0 << 8; next; } };
    say $i;
    $? = 0 << 8;
}

exit $? >> 8;
