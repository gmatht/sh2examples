#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "/tmp/test_deterministic_dir\n";
$? = 0 << 8;

exit $? >> 8;
