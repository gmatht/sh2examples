#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

my $__sh_loop_rc0 = 0;
while (1) {
    print "checking...\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    $CHILD_ERROR = (-f '/tmp/somefile') ? 0 : 1;
    if (-f '/tmp/somefile') {
    } else {
$? = 0 << 8; $CHILD_ERROR = $? >> 8;        last;
    }
    print "file exists\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    $__sh_loop_rc0 = 0; last;
    $__sh_loop_rc0 = $? >> 8;
}
$? = ($__sh_loop_rc0) << 8; $CHILD_ERROR = $? >> 8;
say "exit: $CHILD_ERROR";  # line 11
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
