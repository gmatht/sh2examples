#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $__SH2_AE;

my $j;
my @arr;
my $arr;

$j = "0";  # line 3
@arr = ("a", "b", "c");  # line 4
$j = ($j + (do { my $__d = (do { no warnings; my $__t = (scalar(@arr)); !(defined($__t) && length($__t)) && ($__SH2_AE = 1); $__t }); eval { int((100) / ($__d)) } }));  # line 5
say $j;  # line 6
$? = 0 << 8;
print("arr" . "=[" . (((defined($arr[0]) && length($arr[0])) ? $arr[0] : "")) . "]\n");
$? = 0 << 8;

exit $? >> 8;
