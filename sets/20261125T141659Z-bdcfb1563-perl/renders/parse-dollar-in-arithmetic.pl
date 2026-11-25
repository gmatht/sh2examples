#!/usr/bin/env perl
use strict;
use feature 'say';

our $__SH2_AE;

our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
my $n = '';

$__SH2_AE = 0;  # line 6
do { my @__sh_ae = ((((do { no warnings; my $__t = ($__sh_args[0]); !(defined($__t) && length($__t)) && ($__SH2_AE = 1); $__t }) * 100) + (do { no warnings; my $__t = ($__sh_args[1]); !(defined($__t) && length($__t)) && ($__SH2_AE = 1); $__t }))); if ($__SH2_AE) {$? = 1 << 8; } else { say $__sh_ae[0];$? = 0 << 8; } };
print "n=[$n]\n";
$? = 0 << 8;

exit $? >> 8;
