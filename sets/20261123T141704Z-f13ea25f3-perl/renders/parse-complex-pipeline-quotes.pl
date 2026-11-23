#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $result;

$result = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); my $__sh_pv = do { my $__capf = "/tmp/sh2_cap_1_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { do { my @__o; for my $__l (split(/\n/, 'data', -1)) { $__l =~ s{foo}{bar}g; push @__o, $__l; } print join("\n", @__o), (@__o ? "\n" : ""); $? = 0 << 8; }; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }; do { my @__o; for my $__l (split(/\n/, $__sh_pv, -1)) { my @__f = split(' ', $__l); push @__o, @__f >= 2 ? $__f[2 - 1] : ''; } print join("\n", @__o), (@__o ? "\n" : ""); $? = 0 << 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
say $result;  # line 6
$? = 0 << 8;

exit $? >> 8;
