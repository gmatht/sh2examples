#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);


$? = 0 << 8;  # line 3
if (($__sh_args[0]) =~ /^(?:start)$/) {
    shift @__sh_args;
    $? = 0 << 8;
} elsif (($__sh_args[0]) =~ /^(?:stop)$/) {
    @__sh_args = ("stopped");
    $__sh_args_n = scalar(@__sh_args);
    $? = 0 << 8;
} elsif (($__sh_args[0]) =~ /^(?:check)$/) {
    print "checking\n";
    $? = 0 << 8;
    $? = 0 << 8;
} else {
    print "unknown\n";
    $? = 0 << 8;
    $? = 0 << 8;
}

exit $? >> 8;
