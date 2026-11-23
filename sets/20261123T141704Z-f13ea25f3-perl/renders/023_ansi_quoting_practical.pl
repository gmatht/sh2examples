#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


$? = 0 << 8;  # line 4
print "== Practical examples ==\n";
$? = 0 << 8;
print("Name       Age        City\n");
$? = 0 << 8;
print("John       25         NYC\n");
$? = 0 << 8;
print("Jane       30         LA\n");
$? = 0 << 8;

exit $? >> 8;
