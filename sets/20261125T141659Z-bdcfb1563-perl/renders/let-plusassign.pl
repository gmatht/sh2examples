#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $x;

$x = int($x=5);
$x = int($x+=3);
say join(' ', grep { defined($_) && length($_) } split(/\s+/, $x));  # line 4
$? = 0 << 8;

exit $? >> 8;
