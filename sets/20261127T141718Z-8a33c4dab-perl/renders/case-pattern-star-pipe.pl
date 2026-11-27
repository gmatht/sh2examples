#!/usr/bin/env perl
use strict;
use warnings;
my $cpu;
my $arch = '';
my $basic_machine = '';

$? = 0 << 8;  # line 3
if (($arch) =~ /^(?:i.*86|x86_64)$/) {
    $cpu = $basic_machine;
}
print "cpu=[$cpu]\n";
$? = 0 << 8;

exit $? >> 8;
