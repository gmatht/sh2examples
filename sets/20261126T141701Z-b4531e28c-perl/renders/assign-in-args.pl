#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;

use feature 'say';

$? = (('foo' eq 'bar') ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;
do { my $__x = 'a=b'; $__x =~ s{=}{:}; print STDOUT $__x, "\n"; }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;say "exit: $CHILD_ERROR";  # line 7
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
