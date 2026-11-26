#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "text with 'single quote' inside\n";
$? = 0 << 8;

exit $? >> 8;
