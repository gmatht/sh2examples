#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "\n\n";  # line 4
$? = 0 << 8;
print "hello\tworld\n";  # line 5
$? = 0 << 8;

exit $? >> 8;
