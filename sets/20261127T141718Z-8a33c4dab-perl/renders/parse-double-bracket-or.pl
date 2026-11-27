#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


if (-d '/path1' || -d '/path2' || -d '/path3') {
    print "found\n";
    $? = 0 << 8;
}
say "status: ok";
$? = 0 << 8;

exit $? >> 8;
