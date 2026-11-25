#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
use feature 'say';

my $INTERPRETER_UNDER_TEST;

$INTERPRETER_UNDER_TEST = (do { my $__n = ((defined($__sh_args[0]) && length($__sh_args[0])) ? $__sh_args[0] : "python3"); my $__p = ''; for my $__d (split(/:/, ($ENV{PATH} // ''))) { next if $__d eq ''; my $__c = "$__d/$__n"; if (-f $__c && -x $__c) { $__p = $__c; last } } $__p });  # line 4
if (!($INTERPRETER_UNDER_TEST ne "")) {
    print "Interpreter must be the command line argument.\n";
    $? = 0 << 8;
    exit "4";
}
$ENV{INTERPRETER_UNDER_TEST} = $INTERPRETER_UNDER_TEST;  # line 9
system('bash', '-c', q[${INTERPRETER_UNDER_TEST} '-E' '-' <<'_SH2DOC_'
print('Hello from embedded Python')
_SH2DOC_]);

exit $? >> 8;
