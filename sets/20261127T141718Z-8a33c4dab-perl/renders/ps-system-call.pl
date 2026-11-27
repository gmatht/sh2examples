#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


if ((system('bash', '-c', q{'ps' '-C' 'apt-get,apt,dpkg' > '/dev/null' 2>&1}) == 0)) {
    print "Package manager running\n";
    $? = 0 << 8;
}
print("done: " . (($? >> 8)), "\n");  # line 8
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
