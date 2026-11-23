#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

print "== Here-string with grep -o ==\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
for my $__m (("some pattern here") =~ /pattern/g) { print $__m, "\n"; }$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "exit: $CHILD_ERROR";  # line 9
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
