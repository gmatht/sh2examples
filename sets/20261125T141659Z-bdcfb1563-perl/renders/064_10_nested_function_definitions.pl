#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $__argc = @ARGV;
my $outer_var;
my $outer;
my $inner_var;
my $inner;
my $result;

sub outer_func {  # line 4
    local @ARGV = @_;
    my $__argc = @ARGV;
    my $outer_var = "outer";
        sub inner_func {
        local @ARGV = @_;
        my $__argc = @ARGV;
        my $inner_var = "inner";
        $? = 0 << 8;
        say "Outer: $outer_var, Inner: $inner_var";
        $? = 0 << 8;
        $result = int(($outer_var + $inner_var));
        say "Result: $result";
        $? = 0 << 8;
    }
    inner_func();$? = 0 << 8;
}
outer_func();$? = 0 << 8;  # line 20

exit $? >> 8;
