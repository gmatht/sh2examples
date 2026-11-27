#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';
use English qw(-no_match_vars $ERRNO $EVAL_ERROR $INPUT_RECORD_SEPARATOR $OS_ERROR $PROGRAM_NAME);

our $CHILD_ERROR = 0;

$CHILD_ERROR = (-f '/etc/passwd') ? 0 : 1;
if (-f '/etc/passwd') {
do { my @__lp = do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__f, '<', '/etc/passwd') or do { $CHILD_ERROR = 2; () }; my $__c = <$__f> // ''; close $__f; split(/\n/, $__c) }; pop @__lp if @__lp && $__lp[-1] eq q{}; if (@__lp > 3) { splice(@__lp, 3); } @__lp = map { my @__f = split(':', $_); join(':', $__f[0]) } @__lp; if (@__lp) { print join("\n", @__lp), "\n"; } }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;} else {
    print "not found\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
say "exit: $CHILD_ERROR";  # line 10
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
