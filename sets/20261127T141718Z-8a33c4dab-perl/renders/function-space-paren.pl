#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $__argc = @ARGV;

sub my_func {
    local @ARGV = @_;
    my $__argc = @ARGV;
    return "hello";
}
say my_func();
$? = 0 << 8;
print("done: " . (($? >> 8)), "\n");  # line 9
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
