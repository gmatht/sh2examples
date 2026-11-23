#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);

if (($__sh_args[0]) =~ /^(?:start)$/) {  # line 2
    print "starting\n";
    $? = 0 << 8;
    } elsif (($__sh_args[0]) =~ /^(?:stop)$/) {
    print "stopping\n";
    $? = 0 << 8;
    } else {
    print "unknown\n";
    $? = 0 << 8;
    }

exit $? >> 8;
