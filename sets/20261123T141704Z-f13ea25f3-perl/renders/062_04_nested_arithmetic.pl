#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "Testing nested arithmetic...\n";
$? = 0 << 8;
print("Complex arithmetic: " . (40), "\n");
$? = 0 << 8;

exit $? >> 8;
