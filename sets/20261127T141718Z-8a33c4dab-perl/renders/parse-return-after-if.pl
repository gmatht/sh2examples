#!/usr/bin/env perl
use strict;
use warnings;

print("parsed OK\n");
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 10
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
