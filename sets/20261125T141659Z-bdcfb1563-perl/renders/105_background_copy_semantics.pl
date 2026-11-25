#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $main_exit_code = 0;
our $__sh2_last_bg = '';
my $x;

$x = "1";  # line 6
$| = 1;  # line 7
my $__sh2_bg0 = fork();
$__sh2_last_bg = $__sh2_bg0;
if (defined $__sh2_bg0 && $__sh2_bg0 == 0) {
    $x = "2";
    exit $main_exit_code;
}
while (wait() != -1) {}  # line 8
$? = 0 << 8;
say "x=$x";  # line 9
$? = 0 << 8;

exit $? >> 8;
