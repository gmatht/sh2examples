#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';
use English qw(-no_match_vars $ERRNO $EVAL_ERROR $INPUT_RECORD_SEPARATOR $OS_ERROR $PROGRAM_NAME);

our $CHILD_ERROR = 0;
my $a = '';
my $b = '';

do { my @__lp = do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__f, '<', '/etc/passwd') or do { $CHILD_ERROR = 2; () }; my $__c = <$__f> // ''; close $__f; split(/\n/, $__c) }; pop @__lp if @__lp && $__lp[-1] eq q{}; @__lp = grep { !/^#/ } @__lp; @__lp = map { my @__f = split(':', $_); join(':', @__f[0,2]) } @__lp; @__lp = sort { (split(':', $a))[1] <=> (split(':', $b))[1] } @__lp; if (@__lp > 5) { splice(@__lp, 5); } if (@__lp) { open(my $__ofh, '>', 'users.txt') or die "grep: 'users.txt': $ERRNO"; print $__ofh join("\n", @__lp), "\n"; close $__ofh; } }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;print "Pipeline completed\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "done: $CHILD_ERROR";  # line 7
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
