#!/usr/bin/env perl
use strict;
use warnings;

if (("x") =~ /^(?:\$\(echo "pattern"\))$/) {  # line 3
    open(my $__sh2_rfh0, '>', "/dev/null") or die "Cannot write to /dev/null: $!\n";
    my $__sh2_rsel0 = select($__sh2_rfh0);
    $| = 1;
    my $testvar;
    select($__sh2_rsel0); close($__sh2_rfh0);
}

exit $? >> 8;
