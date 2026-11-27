#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


my $__sh_loop_rc0 = 0;
while (1) {
        print "loop\n";
    $? = 0 << 8;
    $? = 0 << 8;
    $__sh_loop_rc0 = 0; last;
    $__sh_loop_rc0 = $? >> 8;
}
$? = ($__sh_loop_rc0) << 8;
say "status: ok";
$? = 0 << 8;

exit $? >> 8;
