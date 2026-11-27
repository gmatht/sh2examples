#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $__SH2_AE;

my $index;

my $var = '';
my %array;
my $result = '';
my $array = '';

$result = ((do { no warnings; my $__t = (((defined($var) && length($var)) ? $var : "0")); !(defined($__t) && length($__t)) && ($__SH2_AE = 1); $__t }) + (do { no warnings; my $__t = (((defined($array{((defined($index) && length($index)) ? $index : "0")}) && length($array{((defined($index) && length($index)) ? $index : "0")})) ? $array{((defined($index) && length($index)) ? $index : "0")} : "0")); !(defined($__t) && length($__t)) && ($__SH2_AE = 1); $__t }));
say "Eval result: $result";  # line 5
$? = 0 << 8;

exit $? >> 8;
