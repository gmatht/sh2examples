#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


if (-d '/path1' || -d '/path2') {
    print "found\n";
    $? = 0 << 8;
}
print("done: " . (($? >> 8)), "\n");  # line 12
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
