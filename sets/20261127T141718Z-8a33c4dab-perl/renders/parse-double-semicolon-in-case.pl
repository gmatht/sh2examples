#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);

my $dryrun;

$? = 0 << 8;  # line 2
if (($__sh_args[0]) =~ /^(?:--dryrun)$/) {
    $dryrun = "t";
    shift @__sh_args;
    $? = 0 << 8;
} elsif (($__sh_args[0]) =~ /^(?:a)$/) {
    @__sh_args = ("hello");
    $__sh_args_n = scalar(@__sh_args);
    $? = 0 << 8;
} elsif (($__sh_args[0]) =~ /^(?:b)$/) {
    $dryrun = "t";
}
print("parsed OK\n");
$? = 0 << 8;

exit $? >> 8;
