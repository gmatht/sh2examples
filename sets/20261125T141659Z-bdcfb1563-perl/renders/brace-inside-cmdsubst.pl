#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $result;

$result = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); do { my @__o; for my $__l (split(/\n/, 'hello', -1)) { my @__f = split(' ', $__l); push @__o, @__f >= 1 ? $__f[1 - 1] : ''; } print join("\n", @__o), (@__o ? "\n" : ""); $? = 0 << 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
say $result;  # line 5
$? = 0 << 8;

exit $? >> 8;
