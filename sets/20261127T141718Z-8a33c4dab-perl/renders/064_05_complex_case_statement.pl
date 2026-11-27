#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);

$? = 0 << 8;  # line 4
if (($__sh_args[0]) =~ /^(?:[a-z].*)$/) {
    print "Lowercase start\n";
    $? = 0 << 8;
    $? = 0 << 8;
} elsif (($__sh_args[0]) =~ /^(?:[A-Z].*)$/) {
    print "Uppercase start\n";
    $? = 0 << 8;
    $? = 0 << 8;
} elsif (($__sh_args[0]) =~ /^(?:[0-9].*)$/) {
    print "Number start\n";
    $? = 0 << 8;
    $? = 0 << 8;
} elsif (($__sh_args[0]) =~ /^(?:.)$/) {
    print "Single character\n";
    $? = 0 << 8;
    $? = 0 << 8;
} else {
    print "Something else\n";
    $? = 0 << 8;
    $? = 0 << 8;
}

exit $? >> 8;
