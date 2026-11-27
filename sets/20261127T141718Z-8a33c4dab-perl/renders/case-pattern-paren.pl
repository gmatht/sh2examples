#!/usr/bin/env perl
use strict;
use warnings;
my $arch;
my $cpu;

$arch = "x86_64";  # line 3
if (($arch) =~ /^(?:i.*86|x86_64)$/) {
    $cpu = "pc";
} else {
    $cpu = "unknown";
}
print "cpu=[$cpu]\n";
$? = 0 << 8;
$arch = "arm64";  # line 10
$? = 0 << 8;
if (($arch) =~ /^(?:i.*86|x86_64)$/) {
    $cpu = "pc";
} else {
    $cpu = "unknown";
}
print "unknown: cpu=[$cpu]\n";
$? = 0 << 8;

exit $? >> 8;
