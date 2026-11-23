#!/usr/bin/env perl
use strict;
use feature 'say';

our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
my $__argc = @__sh_args;
my $val;

sub upper {  # line 4
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    my $val;
    $val = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); do { my $__t = $__sh_args[0] . "\n"; $__t =~ tr/a-z/A-Z/; print $__t; $? = 0 << 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
    say $val;
    $? = 0 << 8;
}
upper("hello");
upper("world");$? = 0 << 8;

exit $? >> 8;
