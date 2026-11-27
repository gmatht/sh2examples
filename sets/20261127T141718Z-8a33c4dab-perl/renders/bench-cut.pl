#!/usr/bin/env perl
use strict;
use warnings;

do { my @__l = split(/
/, 'abc:def:ghi', -1); pop @__l if @__l && $__l[-1] eq q{}; for my $__l (@__l) { my @__f = split(':', $__l); print STDOUT join(':', $__f[0]), "
"; } }; $? = 0 << 8;do { my @__l = split(/
/, 'abc:def:ghi', -1); pop @__l if @__l && $__l[-1] eq q{}; for my $__l (@__l) { my @__f = split(':', $__l); print STDOUT join(':', @__f[1,2]), "
"; } }; $? = 0 << 8;
exit $? >> 8;
