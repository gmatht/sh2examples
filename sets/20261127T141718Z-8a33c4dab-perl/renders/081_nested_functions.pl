#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $__argc = @ARGV;
my $msg;
my $hello;

sub outer {  # line 3
    local @ARGV = @_;
    my $__argc = @ARGV;
    my $msg = "hello";
        *inner = sub {
        local @ARGV = @_;
        my $__argc = @ARGV;
        say $msg;
        $? = 0 << 8;
    };
    inner();
}
outer();  # line 10

exit $? >> 8;
