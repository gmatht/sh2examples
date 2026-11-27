#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $n;

$n = "5";  # line 4
if (($n == 5)) {
    print "equal\n";
    $? = 0 << 8;
}
print "n=[$n]\n";
$? = 0 << 8;

exit $? >> 8;
