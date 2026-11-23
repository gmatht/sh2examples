#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $result;

$result = do { my $__sh_cap; if (chdir('/tmp')) { $__sh_cap = (require Cwd, Cwd::getcwd()); $? = 0 << 8; } else { $__sh_cap = q{}; $? = 1 << 8; } $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 4
say $result;  # line 5
$? = 0 << 8;

exit $? >> 8;
