#!/usr/bin/env perl
use strict;
use warnings;
my $__argc = @ARGV;

sub show_help {  # line 3
    local @ARGV = @_;
    my $__argc = @ARGV;
    print("Usage: script [options]\n");$? = 0 << 8;
}
show_help();  # line 8
print("exit: " . (($? >> 8)), "\n");  # line 10
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
