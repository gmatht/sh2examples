#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
use feature 'say';

my $__argc = @__sh_args;
my $n;
my $factors;
my $divisor;

print "=== Number Factorization Examples ===\n";
$? = 0 << 8;
sub factorize {
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    my $n = $__sh_args[0];
        my $divisor = "2";
    my $factors = "";
        print "Factors of $n: ";
    $? = 0 << 8;
    my $__sh_loop_rc0 = 0;
    while (int($n) > int(1)) {
        while ((do { my $__a = (do { my $__d = $divisor; eval { int(($n)) % ($__d) } }); defined($__a) ? int($__a) : undef }) == int(0)) {
            if (!($factors ne "")) {
                $factors = $divisor;
            } else {
                $factors = ($factors) . " * " . ($divisor);
            }
            $n = (do { my $__a = (do { my $__d = $divisor; eval { int(($n) / ($__d)) } }); defined($__a) ? int($__a) : undef });
        }
        $divisor = int(($divisor + 1));
        if (int(($divisor * $divisor)) > int($n)) {
            if (int($n) > int(1)) {
                if (!($factors ne "")) {
                    $factors = $n;
                } else {
                    $factors = ($factors) . " * " . ($n);
                }
            }
            $__sh_loop_rc0 = 0; last;
        }
        $__sh_loop_rc0 = $? >> 8;
    }
$? = ($__sh_loop_rc0) << 8;
    say $factors;
    $? = 0 << 8;
}
factorize("12");  # line 44
factorize("28");  # line 45
factorize("100");  # line 46
factorize("12345");  # line 47
print "Factorization complete!\n";
$? = 0 << 8;

exit $? >> 8;
