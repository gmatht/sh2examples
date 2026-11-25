#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $main_exit_code = 0;
our $CHILD_ERROR = 0;
our $__sh2_last_bg = '';

$| = 1;  # line 1
my $__sh2_bg1 = fork();
$__sh2_last_bg = $__sh2_bg1;
if (defined $__sh2_bg1 && $__sh2_bg1 == 0) {
    {
        local %ENV = %ENV;
        sleep("1");
        print "a\n";
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    }
    exit $main_exit_code;
}
print "b\n";  # line 2
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "exit: $CHILD_ERROR";  # line 3
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
