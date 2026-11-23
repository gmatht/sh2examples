#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


$? = 0 << 8;  # line 4
print "== Case modification in parameter expansion ==\n";
$? = 0 << 8;
print "WORLD\n";
$? = 0 << 8;
print "world\n";
$? = 0 << 8;
print "World\n";
$? = 0 << 8;

exit $? >> 8;
