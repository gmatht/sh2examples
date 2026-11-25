#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $name;

$name = "John Doe";
say "Hello John_Doe";
$? = 0 << 8;
say "Length: 8";
$? = 0 << 8;
print("First: " . (join(' ', "John")), "\n");
print("Last: " . (join(' ', substr($name, -3))), "\n");  # line 8
$? = 0 << 8;

exit $? >> 8;
