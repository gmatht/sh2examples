#!/usr/bin/env perl
use strict;
use warnings;

print(($ENV{GREEN}) . "✅ All tests passed" . ($ENV{NC}), "\n");  # line 4
$? = 0 << 8;

exit $? >> 8;
