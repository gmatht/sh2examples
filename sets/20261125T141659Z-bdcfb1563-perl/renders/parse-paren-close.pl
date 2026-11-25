#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $c;

$c = int(13);
print("a=[" . (9) . "]\n");
$? = 0 << 8;
print("b=[" . (4) . "]\n");  # line 6
$? = 0 << 8;
print "c=[$c]\n";  # line 7
$? = 0 << 8;
say int((13 + $c));  # line 8
$? = 0 << 8;

exit $? >> 8;
