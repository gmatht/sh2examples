#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $x;
my $foo = '';

$x = ((defined($foo) && length($foo)) ? $foo : "bar\nbaz");  # line 3
say $x;  # line 5
$? = 0 << 8;

exit $? >> 8;
