#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "echo-line\n";
$? = 0 << 8;
print("printf-line\n");
$? = 0 << 8;

exit $? >> 8;
