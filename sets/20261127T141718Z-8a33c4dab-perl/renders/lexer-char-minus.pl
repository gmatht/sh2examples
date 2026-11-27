#!/usr/bin/env perl
use strict;
use warnings;

$ENV{LANG} = "C";  # line 5
$? = (('a' eq 'a') ? 0 : 1) << 8;
print("parsed OK\n");
$? = 0 << 8;
print("done: " . (($? >> 8)), "\n");  # line 9
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
