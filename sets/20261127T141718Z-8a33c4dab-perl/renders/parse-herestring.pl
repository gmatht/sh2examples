#!/usr/bin/env perl
use strict;
use warnings;

$? = 0 << 8;  # line 3
print("exit: " . (($? >> 8)), "\n");  # line 5
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
