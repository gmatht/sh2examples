#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $a;
my $b;
my $i;
my $temp;

print "=== Fibonacci Sequence (first 20 numbers) ===\n";
$? = 0 << 8;
$a = "0";  # line 9
$b = "1";  # line 10
print "Fibonacci numbers:\n";
$? = 0 << 8;
print "$a $b ";  # line 13
$? = 0 << 8;
my $__sh_loop_rc0 = 0;
for my $__i (("3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20")) {
    $i = $__i;
    $temp = int(($a + $b));
    print "$temp ";
    $? = 0 << 8;
    $a = $b;
    $b = $temp;
    $__sh_loop_rc0 = $? >> 8;
}
$? = ($__sh_loop_rc0) << 8;
print "\n";
$? = 0 << 8;
print "Done!\n";
$? = 0 << 8;

exit $? >> 8;
