#!/usr/bin/env perl
use strict;
use warnings;
my $__argc = @ARGV;

sub INFO {  # line 3
    local @ARGV = @_;
    my $__argc = @ARGV;
    print("hello " . (join(' ', @ARGV)), "\n");
    $? = 0 << 8;
    $? = 0 << 8;
}
INFO("test");

exit $? >> 8;
