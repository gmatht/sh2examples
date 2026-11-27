#!/usr/bin/env perl
use strict;
use warnings;
my $__argc = @ARGV;

sub f {
    local @ARGV = @_;
    my $__argc = @ARGV;
    if (1) {
        $? = 0 << 8;
        print("hello\n");
        $? = 0 << 8;
    }
}
f();  # line 10
print("exit: " . (($? >> 8)), "\n");  # line 12
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
