#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

if ((system('bash', '-c', q{'aa-exec' '--help' > '/dev/null' 2>&1}) == 0)) {
    print "aa-exec works\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
say "exit: $CHILD_ERROR";  # line 7
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
