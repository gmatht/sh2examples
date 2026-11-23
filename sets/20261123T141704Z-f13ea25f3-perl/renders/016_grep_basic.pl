#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;
my $matched;

print "No matches found\n";
do { my $__g = 'HELLO world'; my @__pl = split(/\n/, $__g); my $__m = 0; my $__c = 0; my $__r = qr/hello/; for my $__l (@__pl) { if (lc($__l) =~ $__r) { print $__l, "\n"; $__m = 1; } } $? = ($__m ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; };
do { my $__g = q{line1
line2
line3}; my @__pl = split(/\n/, $__g); my $__m = 0; my $__c = 0; my $__r = qr/line2/; for my $__l (@__pl) { if (!($__l =~ $__r)) { print $__l, "\n"; $__m = 1; } } $? = ($__m ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; };
do { my $__g = q{first
second
third}; my @__pl = split(/\n/, $__g); my $__m = 0; my $__c = 0; my $__r = qr/second/; for my $__i (0..$#__pl) { print(($__i + 1), ":", $__pl[$__i], "\n") if do { my $__l = $__pl[$__i]; $__l =~ $__r }; $__m = 1 if do { my $__l = $__pl[$__i]; $__l =~ $__r }; } $? = ($__m ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; };
do { my $__g = q{match
no match
match again}; my @__pl = split(/\n/, $__g); my $__m = 0; my $__c = 0; my $__r = qr/match/; for my $__l (@__pl) { $__c++ if $__l =~ $__r; } print $__c, "\n"; $__m = $__c > 0 ? 1 : 0; $? = ($__m ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; };
do { my $__g = 'text with pattern123 in it'; my @__pl = split(/\n/, $__g); my $__m = 0; my $__c = 0; my $__r = qr/pattern[0-9]+/; for my $__l (@__pl) { while ($__l =~ /$__r/g) { print $&, "\n"; $__m = 1; } } $? = ($__m ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; };
$matched = do { my $__sh_cap; my $__in = 'test' . "\n"; my $__n = 0; for my $__l (split(/\n/, $__in)) { $__n++ if $__l =~ qr/.*/; } $__sh_cap = $__n; $? = ($__n > 0 ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
say "  grep_exit: $CHILD_ERROR";  # line 25
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "  match_count: $matched";  # line 26
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
