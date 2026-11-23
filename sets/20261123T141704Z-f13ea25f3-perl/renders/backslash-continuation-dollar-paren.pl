#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "hello\nworld\n";
$? = 0 << 8;

exit $? >> 8;
