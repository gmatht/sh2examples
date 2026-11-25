#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

if ((system('bash', '-c', q{'ps' '-C' 'apt-get,apt,dpkg' > '/dev/null' 2>&1}) == 0)) {
    print "Package manager running\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
say "done: $CHILD_ERROR";  # line 8
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
