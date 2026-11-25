#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $all_interfaces;

$all_interfaces = do { my $__sh_cap; $__sh_cap = (do { my @__o; for my $__l (split(/\n/, (do { local $/; if (open(my $__f, '<', '/dev/null')) { my $__c = <$__f>; close $__f; $__c } else { '' } }), -1)) { push @__o, $__l if $__l =~ s{.*}{}; } join("\n", @__o) }); $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 3
say $all_interfaces;  # line 4
$? = 0 << 8;

exit $? >> 8;
