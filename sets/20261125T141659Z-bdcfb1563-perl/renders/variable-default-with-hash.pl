#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "example\n";
$? = 0 << 8;

exit $? >> 8;
