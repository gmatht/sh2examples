#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $i;

$i = int($i=($i + 1));
say join(' ', grep { defined($_) && length($_) } split(/\s+/, $i));  # line 4
$? = 0 << 8;

exit $? >> 8;
