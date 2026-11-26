#!/usr/bin/env perl
use strict;
use feature 'say';

our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
my $__argc = @__sh_args;
my $param1;
my $param2;
my $param3;
my $result;

sub complex_function {
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    my $param1 = $__sh_args[0];
        my $param2 = ((defined($__sh_args[1]) && length($__sh_args[1])) ? $__sh_args[1] : "default");
        my $param3 = (($__sh_args[2] =~ s/"/\\"/gr));
        say "Param1: $param1";
    $? = 0 << 8;
    say "Param2: $param2";
    $? = 0 << 8;
    say "Param3: $param3";
    $? = 0 << 8;
    my $result = join(' ', split(/\s+/, do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n"; do { my $__x = $param1; $__x =~ s{old}{new}g; print STDOUT $__x, "\n"; }; $? = 0 << 8;;; } $__cap =~ s/\n+\z//; $__cap }));
    $? = 0 << 8;
    say "Result: $result";
    $? = 0 << 8;
}
complex_function("test\"quote", "second_param", "third\"param");$? = 0 << 8;

exit $? >> 8;
