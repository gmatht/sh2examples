#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
use feature 'say';

my $input;
my $sanitized;
my $uppercase;

$input = ((defined($__sh_args[0]) && length($__sh_args[0])) ? $__sh_args[0] : "default_value");  # line 4
$sanitized = (($input =~ s/[^a-zA-Z0-9]/_/gr));  # line 5
$uppercase = uc($sanitized);  # line 6
print "Input: '" . ($input) . "' -> Sanitized: '" . ($sanitized) . "' -> Uppercase: '" . ($uppercase) . "'\n";  # line 7
$? = 0 << 8;

exit $? >> 8;
