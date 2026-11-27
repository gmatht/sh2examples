#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


$? = 0 << 8;  # line 4
print "== Unicode and hex ==\n";
$? = 0 << 8;
print "Hello\n";  # line 7
$? = 0 << 8;
print "Hello\n";  # line 8
$? = 0 << 8;

exit $? >> 8;
