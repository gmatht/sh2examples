#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
our $__SH2_AE;

my $n;

open(my $__sh2_rfh0, '>&', \*STDERR) or die "Cannot dup stderr: $!\n";  # line 3
close(STDERR); open(STDERR, '>', "/dev/null") or die "Cannot write to /dev/null: $!\n";
$n = ((do { no warnings; my $__t = ($__sh_args[0]); !(defined($__t) && length($__t)) && ($__SH2_AE = 1); $__t }) + 0);
open(STDERR, '>&', $__sh2_rfh0) or die "Cannot restore stderr: $!\n"; close($__sh2_rfh0);
if ($? == 0) {
    $? = (('($n)' eq '($ARGV[0])') ? 0 : 1) << 8;
}
print "n=[$n]\n";
$? = 0 << 8;

exit $? >> 8;
