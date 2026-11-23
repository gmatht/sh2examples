#!/usr/bin/env perl
use strict;
use warnings;
my $IFS;

print("$1\n");$? = 0 << 8;  # line 4
$IFS = "\n";  # line 7
$ENV{IFS} = ${IFS} if exists $ENV{IFS};
print("IFS=[\n]\n");
$? = 0 << 8;

exit $? >> 8;
