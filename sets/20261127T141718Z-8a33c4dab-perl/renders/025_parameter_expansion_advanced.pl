#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


$? = 0 << 8;  # line 4
print "== Advanced parameter expansion ==\n";
$? = 0 << 8;
print "025_param_expansion_file.txt\n";
$? = 0 << 8;
print "/tmp\n";
$? = 0 << 8;
print "aXXa\n";
$? = 0 << 8;

exit $? >> 8;
