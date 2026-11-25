#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;

use feature 'say';

my $__sh_cap; do { $__sh_cap = (do { require File::Basename; File::Basename::basename(do { my $__sh_cap; $__sh_cap = (require Cwd, Cwd::getcwd()); $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap }) }); $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; print $__sh_cap, "\n" if defined $__sh_cap;  # line 1
say "exit: $CHILD_ERROR";  # line 3
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
