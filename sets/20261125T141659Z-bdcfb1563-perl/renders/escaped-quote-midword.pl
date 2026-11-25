#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "x 'y\n";  # line 7
$? = 0 << 8;

exit $? >> 8;
