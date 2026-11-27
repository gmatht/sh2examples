#!/usr/bin/env perl
use strict;
use warnings;

$ENV{ENV} = "";  # line 3
$? = 0 << 8;
print("ENV=[" . ($ENV{ENV}) . "]\n");  # line 4
$? = 0 << 8;

exit $? >> 8;
