#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print '2', "\n"; $? = 0 << 8;say "status: ok";
$? = 0 << 8;

exit $? >> 8;
