#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "abc=def\n";
$? = 0 << 8;

exit $? >> 8;
