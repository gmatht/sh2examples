#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my @arr;
my $x;
my $arr;

print "== Indexed arrays ==\n";
$? = 0 << 8;
@arr = ("one", "two", "three");  # line 7
say $arr[1];  # line 8
say scalar(@arr);
$? = 0 << 8;
for my $__x ((@arr)) {
    $x = $__x;
    print "$x ";
    $? = 0 << 8;
}
print "\n";  # line 10
$? = 0 << 8;

exit $? >> 8;
