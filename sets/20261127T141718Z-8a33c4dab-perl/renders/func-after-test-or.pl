#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $__argc = @ARGV;

$? = (($ENV{VAR} ne "") ? 0 : 1) << 8;
if (($? >> 8) != 0) {
    $ENV{VAR} = "default";
}
sub my_func {
    local @ARGV = @_;
    my $__argc = @ARGV;
    return "hello";
}
say my_func();
$? = 0 << 8;

exit $? >> 8;
