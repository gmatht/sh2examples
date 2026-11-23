#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $__argc = @ARGV;

sub f {
    local @ARGV = @_;
    my $__argc = @ARGV;
    return "f-called";
}
say f();
say f();
$? = 0 << 8;

exit $? >> 8;
