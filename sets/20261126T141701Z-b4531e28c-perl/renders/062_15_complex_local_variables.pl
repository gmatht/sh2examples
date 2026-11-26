#!/usr/bin/env perl
use strict;
use feature 'say';

our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
my $__argc = @__sh_args;
my $var1;
my $var2;
my $var3;

sub test_locals {  # line 4
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    my $var1 = $__sh_args[0];
        my $var2 = ((defined($__sh_args[1]) && length($__sh_args[1])) ? $__sh_args[1] : "default_value");
        my $var3 = do { my $__cap = do { my $__t = $var1 . "\n"; $__t =~ tr/a-z/A-Z/; $__t }; $__cap =~ s/\n+\z//; $__cap };
    $? = 0 << 8;
    say "Var1: $var1";
    $? = 0 << 8;
    say "Var2: $var2";
    $? = 0 << 8;
    say "Var3: $var3";
    $? = 0 << 8;
}
test_locals("hello", "world");$? = 0 << 8;

exit $? >> 8;
