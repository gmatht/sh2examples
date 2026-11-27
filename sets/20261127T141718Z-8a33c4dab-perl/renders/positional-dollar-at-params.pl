#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $__argc = @ARGV;
my $a;

sub show {  # line 3
    local @ARGV = @_;
    my $__argc = @ARGV;
    print("count=" . (scalar(@ARGV)), "\n");
    $? = 0 << 8;
        print("all=" . (join(' ', @ARGV)), "\n");
    $? = 0 << 8;
        my $__sh_loop_rc0 = 0;
    for my $__a ((@ARGV)) {
        $a = $__a;
        say "arg=$a";
        $? = 0 << 8;
        $__sh_loop_rc0 = $? >> 8;
    }
$? = ($__sh_loop_rc0) << 8;
}
show("one", "two three", "four");

exit $? >> 8;
