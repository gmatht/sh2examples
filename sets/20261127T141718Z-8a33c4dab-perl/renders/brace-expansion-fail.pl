#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my @var;

print "1 2 3 4 5\n";
$? = 0 << 8;
say @var;  # line 5
$? = 0 << 8;

exit $? >> 8;
