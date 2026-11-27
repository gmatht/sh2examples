#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $name;

print("hi 42\n");
$? = 0 << 8;
print("a\tb\n");
$? = 0 << 8;
$name = "foo-bar";
$? = 0 << 8;
say "name=$name";  # line 6
$? = 0 << 8;

exit $? >> 8;
