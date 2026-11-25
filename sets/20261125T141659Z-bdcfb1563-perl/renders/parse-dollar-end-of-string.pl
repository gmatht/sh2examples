#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "hello\$\n";  # line 4
$? = 0 << 8;
print "world\$\n";  # line 5
$? = 0 << 8;
print "\$\n";
$? = 0 << 8;

exit $? >> 8;
