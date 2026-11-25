#!/usr/bin/env perl
use strict;
use warnings;
my $__sh2_tc0;

$__sh2_tc0 = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n"; do { my @__gl = split(/\n/, 'test', -1); pop @__gl if @__gl && $__gl[-1] eq q{}; my @__mi; for my $__i (0..$#__gl) { if ($__gl[$__i] =~ /test/) { push @__mi, $__i; } } my @__out; my %__se; for my $__i (@__mi) { my $__lo = $__i - 0; $__lo = 0 if $__lo < 0; my $__hi = $__i + 0; $__hi = $#__gl if $__hi > $#__gl; for my $__j ($__lo..$__hi) { push @__out, $__gl[$__j] unless $__se{$__j}++; } } if (@__out) { print join("\n", @__out), "\n"; } $? = 0 << 8; };
;; } $__cap =~ s/\n+\z//; $__cap };
if (length($__sh2_tc0) && 1) {
    print "found\n";
    $? = 0 << 8;
}

exit $? >> 8;
