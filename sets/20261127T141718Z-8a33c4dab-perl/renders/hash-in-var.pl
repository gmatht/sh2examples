#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $x;
my $y = '';

$x = (($y =~ s{^.*? }{}r));  # line 3
say $x;  # line 4
$? = 0 << 8;

exit $? >> 8;
