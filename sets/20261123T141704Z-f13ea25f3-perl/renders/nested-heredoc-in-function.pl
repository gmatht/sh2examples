#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;
use feature 'say';

my $__argc = @ARGV;

sub show_help {  # line 3
    local @ARGV = @_;
    my $__argc = @ARGV;
    print("Usage: script [options]\n");$? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
show_help();$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 8
say "exit: $CHILD_ERROR";  # line 10
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
