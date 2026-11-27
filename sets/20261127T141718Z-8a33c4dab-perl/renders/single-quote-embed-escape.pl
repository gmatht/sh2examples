#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "\n  s/'/'\\\\''/g\n\n";
$? = 0 << 8;

exit $? >> 8;
