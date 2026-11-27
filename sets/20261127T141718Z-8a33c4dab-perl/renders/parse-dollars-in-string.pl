#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $i;

for my $__i ((@ARGV)) {  # line 2
    $i = $__i;
    say $i;
    $? = 0 << 8;
}
my $__sh_loop_rc0 = 0;  # line 3
for my $__i (join(' ', @ARGV)) {
    $i = $__i;
    say $i;
    $? = 0 << 8;
    $__sh_loop_rc0 = $? >> 8;
}
$? = ($__sh_loop_rc0) << 8;

exit $? >> 8;
