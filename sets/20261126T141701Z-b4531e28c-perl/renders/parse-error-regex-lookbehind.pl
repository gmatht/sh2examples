#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;

use feature 'say';

do { my $__t = 'hello' . "\n"; $__t =~ tr/h/H/; print $__t; $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
say "exit: $CHILD_ERROR";  # line 5
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
