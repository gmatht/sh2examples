#!/usr/bin/env perl
use strict;
use warnings;

print("exit: " . (($? >> 8)), "\n");  # line 2
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
