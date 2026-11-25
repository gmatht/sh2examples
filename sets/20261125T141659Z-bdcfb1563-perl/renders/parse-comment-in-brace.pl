#!/usr/bin/env perl
use strict;
use warnings;
my $tmp;
my $opt = '';
my $configfile = '';

if (($opt) =~ /^(?:)$/) {  # line 3
} elsif (($opt) =~ /^(?:configfile=.*)$/) {
    $tmp = (($opt =~ s{^.*?=}{}r));
} else {
    print "default\n";
    $? = 0 << 8;
    }
print "tmp=[$tmp]\n";
$? = 0 << 8;
print "configfile=[$configfile]\n";
$? = 0 << 8;

exit $? >> 8;
