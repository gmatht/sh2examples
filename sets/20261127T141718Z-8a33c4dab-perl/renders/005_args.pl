#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $a;

print "== Argument count ==\n";
$? = 0 << 8;
say scalar(@ARGV);  # line 7
$? = 0 << 8;
print "== Arguments ==\n";
$? = 0 << 8;
my $__sh_loop_rc0 = 0;  # line 10
for my $__a ((@ARGV)) {
    $a = $__a;
    say "Arg: $a";
    $? = 0 << 8;
    $__sh_loop_rc0 = $? >> 8;
}
$? = ($__sh_loop_rc0) << 8;

exit $? >> 8;
