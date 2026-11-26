#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;
use feature 'say';

my $__argc = @ARGV;

sub f {
    local @ARGV = @_;
    my $__argc = @ARGV;
    if (1) {
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        print("hello\n");
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    }
}
f();$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 10
say "exit: $CHILD_ERROR";  # line 12
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
