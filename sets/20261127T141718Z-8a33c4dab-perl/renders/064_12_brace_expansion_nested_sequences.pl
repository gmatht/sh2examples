#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


do { my $__ok = 1; for my $__d ('project/src/main/java', 'project/src/main/resources', 'project/src/test/java', 'project/src/test/resources', 'project/docs/api', 'project/docs/user', 'project/build/classes', 'project/build/lib') { my $__acc = $__d =~ m{^/} ? '' : '.'; for my $__c (split m{/+}, $__d) { next if $__c eq '' || $__c eq '.'; $__acc .= "/$__c"; next if -d $__acc; unless (mkdir($__acc)) { $__ok = 0; print STDERR "mkdir: $!\n"; last; } } } $? = ($__ok ? 0 : 1) << 8; };
print "Project structure created\n";
$? = 0 << 8;
print("done: " . (($? >> 8)), "\n");  # line 7
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
