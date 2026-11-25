#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


$? = 0 << 8;  # line 4
print "== ANSI-C quoting ==\n";
$? = 0 << 8;
print "line1\nline2\tTabbed\n";  # line 7
$? = 0 << 8;

exit $? >> 8;
