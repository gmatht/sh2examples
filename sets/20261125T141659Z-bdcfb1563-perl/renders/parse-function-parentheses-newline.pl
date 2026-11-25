#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;
use feature 'say';

my $__argc = @ARGV;

sub myfunc {
    local @ARGV = @_;
    my $__argc = @ARGV;
    return "hello";
}
say myfunc();
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "exit: $CHILD_ERROR";  # line 9
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
