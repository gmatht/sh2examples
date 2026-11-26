#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;
use feature 'say';

my $__argc = @ARGV;

$? = (($ENV{VAR} ne "") ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;
if ($CHILD_ERROR != 0) {
    $ENV{VAR} = "default";
}
sub my_func {
    local @ARGV = @_;
    my $__argc = @ARGV;
    return "hello " . ($ENV{VAR});
}
say my_func();
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
