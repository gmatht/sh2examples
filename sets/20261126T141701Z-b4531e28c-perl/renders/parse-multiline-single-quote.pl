#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "\nmulti-line\nvalue\n\n";
$? = 0 << 8;

exit $? >> 8;
