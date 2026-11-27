#!/usr/bin/env perl
use strict;
use warnings;

print("sub=" . (join(' ', "cde")), "\n");
$? = 0 << 8;

exit $? >> 8;
