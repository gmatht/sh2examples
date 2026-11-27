#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $__argc = @ARGV;

sub exitScript {
    local @ARGV = @_;
    my $__argc = @ARGV;
    return "called exitScript";
}
say exitScript("1");
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 6
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
