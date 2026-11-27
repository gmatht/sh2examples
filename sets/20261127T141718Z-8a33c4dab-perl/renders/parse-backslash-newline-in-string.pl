#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "/tmp/test_deterministic.txt\n";
$? = 0 << 8;

exit $? >> 8;
