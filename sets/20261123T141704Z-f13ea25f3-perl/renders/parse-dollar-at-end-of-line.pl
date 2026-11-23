#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "the price is \$\n";  # line 4
$? = 0 << 8;
print "\$\n";
$? = 0 << 8;

exit $? >> 8;
