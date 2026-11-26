#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);

if (($__sh_args[0]) =~ /^(?:a)$/) {  # line 3
    if (($__sh_args[1]) =~ /^(?:x)$/) {
        print "x\n";
        $? = 0 << 8;
            }
} elsif (($__sh_args[0]) =~ /^(?:b)$/) {
    print "b\n";
    $? = 0 << 8;
    }

exit $? >> 8;
