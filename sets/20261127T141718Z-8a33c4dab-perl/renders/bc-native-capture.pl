#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $n;
my $sum;
my $i;

print "5\n";
$? = 0 << 8;
print "2.50\n";
$? = 0 << 8;
print "1024\n";
$? = 0 << 8;
print "1\n";
$? = 0 << 8;
print "4\n";
$? = 0 << 8;
print ".75\n";
$? = 0 << 8;
print "5\n";
$? = 0 << 8;
for my $__n (("4", "9", "16", "25", "100")) {  # line 17
    $n = $__n;
    print("sqrt(" . ($n) . ") = " . (do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); print int(sqrt(($n))), "\n"; $? = 0; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap }), "\n");
    }
$sum = "0";  # line 21
my $__sh_loop_rc0 = 0;  # line 22
for my $__i (("1", "2", "3", "4", "5")) {
    $i = $__i;
    $sum = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); print int(($sum) + ($i)), "\n"; $? = 0; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
    $__sh_loop_rc0 = $? >> 8;
}
$? = ($__sh_loop_rc0) << 8;
say "sum=$sum";  # line 25
$? = 0 << 8;

exit $? >> 8;
