#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $count;

$count = "3";
my $__sh_loop_rc0 = 0;
while (!(int($count) == int(0))) {
    say "count=$count";
    $? = 0 << 8;
    $count = int(($count - 1));
    $__sh_loop_rc0 = $? >> 8;
}
$? = ($__sh_loop_rc0) << 8;

exit $? >> 8;
