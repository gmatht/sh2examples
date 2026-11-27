#!/usr/bin/env perl
use strict;
use warnings;

$? = 0 << 8;  # line 3
$? = 0 << 8;  # line 4
print("parsed OK\n");
$? = 0 << 8;
print("done: " . (($? >> 8)), "\n");  # line 7
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
