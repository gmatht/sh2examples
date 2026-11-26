#!/usr/bin/env perl
use strict;
use warnings;
my $IFS;

$IFS = "\n\t";  # line 3
$ENV{IFS} = ${IFS} if exists $ENV{IFS};
print("IFS value (hex)=");  # line 4
$? = 0 << 8;
print("\$'\\n\\t'");
$? = 0 << 8;
print("\n");  # line 4
$? = 0 << 8;

exit $? >> 8;
