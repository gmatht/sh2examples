#!/usr/bin/env perl
use strict;
use feature 'say';

our $__SH2_AE;

our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
my $__argc = @__sh_args;
my $i;

sub f {
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    return ((do { no warnings; my $__t = ($__sh_args[0]); !(defined($__t) && length($__t)) && ($__SH2_AE = 1); $__t }) + (do { no warnings; my $__t = ($__sh_args[0]); !(defined($__t) && length($__t)) && ($__SH2_AE = 1); $__t }));
}
$i = "0";
while (int($i) < int(5)) {
    say f($i);
    $? = 0 << 8;
    $i = int(($i + 1));
}

exit $? >> 8;
