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
for my $__a ((@ARGV)) {  # line 10
    $a = $__a;
    say "Arg: $a";
    $? = 0 << 8;
}

exit $? >> 8;
