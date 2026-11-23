#!/usr/bin/env perl
use strict;
use feature 'say';

our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);

if (($__sh_args[0]) =~ /^(?:on)$/) {  # line 2
    say join(' ', "Please", "rerun", "\\`" . ($0), "on'");
    }

exit $? >> 8;
