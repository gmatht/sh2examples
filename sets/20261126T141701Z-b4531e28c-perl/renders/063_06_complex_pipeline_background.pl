#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $main_exit_code = 0;
our $CHILD_ERROR = 0;
our $__sh2_last_bg = '';

$| = 1;  # line 4
my $__sh2_bg2 = fork();
$__sh2_last_bg = $__sh2_bg2;
if (defined $__sh2_bg2 && $__sh2_bg2 == 0) {
    {
        local %ENV = %ENV;
        sleep("1");
        print "Starting\n";
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
            }
    exit $main_exit_code;
}
$| = 1;  # line 5
my $__sh2_bg3 = fork();
$__sh2_last_bg = $__sh2_bg3;
if (defined $__sh2_bg3 && $__sh2_bg3 == 0) {
    {
        local %ENV = %ENV;
        sleep("2");
        print "Processing\n";
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
            }
    exit $main_exit_code;
}
while (wait() != -1) {}  # line 6
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "All done\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "exit: $CHILD_ERROR";  # line 9
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
