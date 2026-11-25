#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


say "len=6";
$? = 0 << 8;

exit $? >> 8;
