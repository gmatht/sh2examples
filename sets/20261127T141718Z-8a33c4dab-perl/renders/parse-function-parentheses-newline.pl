#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $__argc = @ARGV;

sub myfunc {
    local @ARGV = @_;
    my $__argc = @ARGV;
    return "hello";
}
say myfunc();
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 9
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
