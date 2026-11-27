#!/usr/bin/env perl
use strict;
use warnings;

print("a [Y/n] b\n");
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 6
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
