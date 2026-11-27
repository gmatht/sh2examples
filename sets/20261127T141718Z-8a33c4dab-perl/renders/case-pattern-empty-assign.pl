#!/usr/bin/env perl
use strict;
use warnings;
my $x = '';

$? = 0 << 8;
if (($x) =~ /^(?:a)$/) {
} else {
}
print("parsed OK\n");
$? = 0 << 8;

exit $? >> 8;
