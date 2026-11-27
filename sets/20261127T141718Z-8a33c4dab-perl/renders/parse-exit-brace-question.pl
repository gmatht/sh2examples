#!/usr/bin/env perl
use strict;
use warnings;

exit ($? >> 8);  # line 3
print("parsed OK\n");
$? = 0 << 8;

exit $? >> 8;
