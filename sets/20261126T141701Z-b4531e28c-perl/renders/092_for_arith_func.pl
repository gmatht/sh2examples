#!/usr/bin/env perl
use strict;
use feature 'say';

our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
my $__argc = @__sh_args;
my $n;
my $i;
my $result;

sub factorial {
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    my $n = $__sh_args[0];
        my $result = "1";
    my $i;
    $i = int($i=2);
    while (($i <= $n)) {
        $result = int(($result * $i));
        $? = (((($i++)) != 0) ? 0 : 1) << 8;
    }
    return $result;
}
say factorial("5");
say factorial("6");
$? = 0 << 8;

exit $? >> 8;
