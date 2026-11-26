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
my $result;

sub complex_func {  # line 4
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    my $x = $__sh_args[0];
        my $y = $__sh_args[1];
        my $result = int(($x + $y));
    $? = 0 << 8;
    say "Sum: $result";
    $? = 0 << 8;
    say "Args: $x $y";
    $? = 0 << 8;
}
complex_func("3", "7");  # line 13
complex_func("10", "20");$? = 0 << 8;  # line 14

exit $? >> 8;
