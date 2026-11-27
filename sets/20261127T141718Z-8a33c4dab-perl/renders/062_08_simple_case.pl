#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
use feature 'say';


print "Testing simple case patterns...\n";
$? = 0 << 8;
$? = 0 << 8;  # line 5
if (($__sh_args[0]) =~ /^(?:test)$/) {
    print "Matched test\n";
    $? = 0 << 8;
    $? = 0 << 8;
} else {
    print "Default case\n";
    $? = 0 << 8;
    $? = 0 << 8;
}

exit $? >> 8;
