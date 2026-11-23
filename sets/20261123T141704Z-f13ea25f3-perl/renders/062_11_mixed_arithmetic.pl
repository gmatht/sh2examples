#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "Testing mixed arithmetic...\n";
$? = 0 << 8;
print("Mixed base result: " . (776), "\n");
$? = 0 << 8;

exit $? >> 8;
