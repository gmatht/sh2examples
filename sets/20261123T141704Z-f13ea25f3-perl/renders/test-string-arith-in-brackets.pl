#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my @a;
my $a;

@a = ("10", "20", "30", "40");  # line 3
if (int(1) < int((${a}[1] + ${a}[2]))) {
    print "arith-test-ok\n";
    $? = 0 << 8;
} else {
    print "arith-test-bad\n";
    $? = 0 << 8;
}
print "done\n";  # line 6
$? = 0 << 8;

exit $? >> 8;
