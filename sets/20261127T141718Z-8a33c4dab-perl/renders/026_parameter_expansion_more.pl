#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


$? = 0 << 8;  # line 4
print "== More parameter expansion ==\n";
$? = 0 << 8;
print " world\n";
$? = 0 << 8;
print "hello \n";
$? = 0 << 8;
print "hell0 w0rld\n";
$? = 0 << 8;

exit $? >> 8;
