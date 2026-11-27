#!/usr/bin/env perl
use strict;
use warnings;

my $__sh_loop_rc0 = 0;
while (1) {
    $? = 0 << 8;
    last;
    if (($? >> 8) != 0) {
        exit "1";
    }
    $__sh_loop_rc0 = $? >> 8;
}
$? = ($__sh_loop_rc0) << 8;
print("parsed OK\n");
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 5
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
