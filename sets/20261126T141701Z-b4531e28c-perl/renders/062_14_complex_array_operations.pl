#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my @array;
my $array;

print "Testing complex array operations...\n";
$? = 0 << 8;
@array = ("item1", "item2", "item3");
push @array, ("item4");
print("Array: " . (join(' ', @array)), "\n");  # line 7
print("Length: " . (join(' ', scalar(@array))), "\n");
print("First item: " . ($array[0]), "\n");  # line 9
$? = 0 << 8;

exit $? >> 8;
