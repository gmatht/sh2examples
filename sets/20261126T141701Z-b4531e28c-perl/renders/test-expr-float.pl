#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

$CHILD_ERROR = ((do { my $__sh_cap; $__sh_cap = (require POSIX, (POSIX::uname())[2]); $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap } =~ /^(?s)5\.4\..*$/)) ? 0 : 1;
if ((do { my $__sh_cap; $__sh_cap = (require POSIX, (POSIX::uname())[2]); $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap } =~ /^(?s)5\.4\..*$/)) {
    print "match\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    $CHILD_ERROR = 0;
}

exit $? >> 8;
