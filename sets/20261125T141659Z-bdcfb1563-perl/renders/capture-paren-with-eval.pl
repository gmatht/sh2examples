#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


say 3;
$? = 0 << 8;

exit $? >> 8;
