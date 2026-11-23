#!/usr/bin/env perl
use strict;
use warnings;
my $result;

$result = (do { print STDERR "$0: syntax error in expression (\"echo \\\"test\\\"\")\n"; undef });  # line 3
print("result=" . (((defined($result) && length($result)) ? $result : "empty")) . "\n");  # line 4
$? = 0 << 8;

exit $? >> 8;
