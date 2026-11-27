#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
my $__argc = @__sh_args;
my $s;
my $i;
my $len;
my $ch;

sub print_chars {
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    $s = $__sh_args[0];
    $i = "0";
    $len = $__sh_args[1];
    my $__sh_loop_rc0 = 0;
    while (int($i) < int($len)) {
        $ch = substr($s, $i, 1);
        print($ch);
        $? = 0 << 8;
        $i = int(($i + 1));
        $__sh_loop_rc0 = $? >> 8;
    }
$? = ($__sh_loop_rc0) << 8;
    print "\n";
    $? = 0 << 8;
}
print_chars("HELLOWORLD", "10");

exit $? >> 8;
