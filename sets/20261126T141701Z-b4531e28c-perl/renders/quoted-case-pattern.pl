#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);

if (($__sh_args[0]) =~ /^(?:test-pattern)$/) {  # line 1
    print "match\n";
    $? = 0 << 8;
    }

exit $? >> 8;
