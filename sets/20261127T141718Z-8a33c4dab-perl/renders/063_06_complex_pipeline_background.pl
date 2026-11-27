#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $main_exit_code = 0;
our $__sh2_last_bg = '';

$| = 1;  # line 4
my $__sh2_bg0 = fork();
$__sh2_last_bg = $__sh2_bg0;
if (defined $__sh2_bg0 && $__sh2_bg0 == 0) {
    {
        local %ENV = %ENV;
        sleep("1");
        print "Starting\n";
        $? = 0 << 8;
            }
    exit $main_exit_code;
}
$| = 1;  # line 5
my $__sh2_bg1 = fork();
$__sh2_last_bg = $__sh2_bg1;
if (defined $__sh2_bg1 && $__sh2_bg1 == 0) {
    {
        local %ENV = %ENV;
        sleep("2");
        print "Processing\n";
        $? = 0 << 8;
            }
    exit $main_exit_code;
}
while (wait() != -1) {}  # line 6
$? = 0 << 8;
print "All done\n";
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 9
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
