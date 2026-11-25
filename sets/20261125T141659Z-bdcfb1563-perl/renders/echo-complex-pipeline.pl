#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $result;

$result = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); do { my $__t = 'test' . "\n"; $__t =~ tr/a-z/A-Z/; print $__t; $? = 0 << 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
say $result;  # line 4
$? = 0 << 8;

exit $? >> 8;
