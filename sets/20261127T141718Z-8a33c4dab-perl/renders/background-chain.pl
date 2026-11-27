#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $main_exit_code = 0;
our $__sh2_last_bg = '';

$| = 1;  # line 3
my $__sh2_bg0 = fork();
$__sh2_last_bg = $__sh2_bg0;
if (defined $__sh2_bg0 && $__sh2_bg0 == 0) {
    print "start\n";
    $? = 0 << 8;
        exit $main_exit_code;
}
while (wait() != -1) {}  # line 3
$? = 0 << 8;
print "done\n";  # line 4
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 5
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
