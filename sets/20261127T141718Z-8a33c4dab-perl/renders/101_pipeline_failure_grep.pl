#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "Grep test:\n";
$? = 0 << 8;
do { my $__g = 'alpha beta gamma'; my $__m = 0; for my $__l (split(/\n/, $__g)) { if ($__l =~ /beta/) { print $__l, "\n"; $__m = 1; } } $? = ($__m ? 0 : 1) << 8; };
print "---\n";
$? = 0 << 8;
do { my $__g = 'alpha beta gamma'; my @__pl = split(/\n/, $__g); my $__m = 0; my $__c = 0; my $__r = qr/beta/; for my $__l (@__pl) { while ($__l =~ /$__r/g) { print $&, "\n"; $__m = 1; } } $? = ($__m ? 0 : 1) << 8; };
print "done\n";
$? = 0 << 8;

exit $? >> 8;
