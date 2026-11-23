#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $os;

$os = do { my $__sh_cap; $__sh_cap = (require POSIX, (POSIX::uname())[0]); $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 3
say $os;  # line 4
$? = 0 << 8;

exit $? >> 8;
