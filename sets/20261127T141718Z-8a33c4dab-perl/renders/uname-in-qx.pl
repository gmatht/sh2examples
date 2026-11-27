#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $system;

$system = do { my $__sh_cap; $__sh_cap = (require POSIX, (POSIX::uname())[0]); $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 4
say $system;  # line 5
$? = 0 << 8;

exit $? >> 8;
