#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


my $__sh_loop_rc0 = 0;
while (!(1)) {
    print "waiting\n";
    $? = 0 << 8;
    $__sh_loop_rc0 = $? >> 8;
}
$? = ($__sh_loop_rc0) << 8;
print("exit: " . (($? >> 8)), "\n");  # line 7
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
