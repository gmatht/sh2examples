#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $var = '';

say int(($var + 1));  # line 4
$? = 0 << 8;

exit $? >> 8;
