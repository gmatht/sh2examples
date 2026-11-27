#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


if ((system('bash', '-c', q{'aa-exec' '--help' > '/dev/null' 2>&1}) == 0)) {
    print "aa-exec works\n";
    $? = 0 << 8;
}
print("exit: " . (($? >> 8)), "\n");  # line 7
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
