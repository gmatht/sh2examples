#!/usr/bin/env perl
use strict;
use feature 'say';

our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
my $__argc = @__sh_args;

sub greet {
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    return "Hello, " . ($__sh_args[0]) . "!";
}
say greet("World");
$? = 0 << 8;

exit $? >> 8;
