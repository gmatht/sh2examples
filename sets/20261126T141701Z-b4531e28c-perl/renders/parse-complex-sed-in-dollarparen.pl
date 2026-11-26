#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $x;

$x = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n"; do { my $__x = 'foo'; $__x =~ s{foo}{bar}; print STDOUT $__x, "\n"; }; $? = 0 << 8;;; } $__cap =~ s/\n+\z//; $__cap };  # line 3
say $x;  # line 4
$? = 0 << 8;

exit $? >> 8;
