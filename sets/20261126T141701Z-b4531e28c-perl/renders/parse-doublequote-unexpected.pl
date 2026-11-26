#!/usr/bin/env perl
use strict;
use warnings;

print("OPTIONS_SPEC=[--branch=<newname> ]\n");
$? = 0 << 8;

exit $? >> 8;
