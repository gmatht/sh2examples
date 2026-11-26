#!/usr/bin/env perl
use strict;
use warnings;

print("OPTIONS_SPEC=[--help --version ]\n");
$? = 0 << 8;

exit $? >> 8;
