#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);

print "Invalid configuration " . "`" . ($__sh_args[0]) . "\\': more than four components\n";  # line 3
$? = 0 << 8;
exit "1";  # line 4

exit $? >> 8;
