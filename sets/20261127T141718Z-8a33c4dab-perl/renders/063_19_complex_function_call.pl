#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $__argc = @ARGV;
my $var = '';

sub complex_function {
    local @ARGV = @_;
    my $__argc = @ARGV;
    return "fn args: " . (join(' ', @ARGV));
}
say complex_function("--long-option=value with spaces", "--array-option", "item1", "item2", "item3", "--flag", "positional argument", ((defined($var) && length($var)) ? $var : "default"), "computed");
$? = 0 << 8;

exit $? >> 8;
