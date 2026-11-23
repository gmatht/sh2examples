#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


say "id: \$42";
$? = 0 << 8;
print "price: \$5.00\n";
$? = 0 << 8;

exit $? >> 8;
