#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


say "cmdsub=sub-result";
$? = 0 << 8;
{  # line 5
    local %ENV = %ENV;
    print "group-run\n";
    $? = 0 << 8;
    $? = 0 << 8;
}

exit $? >> 8;
