#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);

if (($__sh_args[0]) =~ /^(?:start)$/) {  # line 4
    print "Starting...\n";
    $? = 0 << 8;
        if (($__sh_args[1]) =~ /^(?:fast)$/) {
        print "Fast start\n";
        $? = 0 << 8;
            } elsif (($__sh_args[1]) =~ /^(?:slow)$/) {
        print "Slow start\n";
        $? = 0 << 8;
            }
} elsif (($__sh_args[0]) =~ /^(?:stop)$/) {
    print "Stopping...\n";
    $? = 0 << 8;
    } elsif (($__sh_args[0]) =~ /^(?:restart)$/) {
    print "Restarting...\n";
    $? = 0 << 8;
        if (($__sh_args[1]) =~ /^(?:soft)$/) {
        print "Soft restart\n";
        $? = 0 << 8;
            } elsif (($__sh_args[1]) =~ /^(?:hard)$/) {
        print "Hard restart\n";
        $? = 0 << 8;
            }
}

exit $? >> 8;
