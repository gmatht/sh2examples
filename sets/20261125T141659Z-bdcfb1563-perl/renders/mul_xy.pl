#!/usr/bin/env perl
use strict;
use feature 'say';

our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
my $__argc = @__sh_args;
my $x;
my $y;

sub mul_xy {
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    $x = $__sh_args[0];
    $y = $__sh_args[1];
    return int(($x * $y));
}
say mul_xy("3", "4");
$? = 0 << 8;

exit $? >> 8;
