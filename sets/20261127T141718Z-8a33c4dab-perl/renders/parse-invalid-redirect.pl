#!/usr/bin/env perl
use strict;
use warnings;

system('bash', '-c', q{'exec' 3>&1});  # line 3
print "test\n";
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 6
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
