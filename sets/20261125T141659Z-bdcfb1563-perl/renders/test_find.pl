#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;

use feature 'say';

do { my @__out; my $__lim = 3; my $__walk; $__walk = sub { my ($__d) = @_; return if @__out >= $__lim; opendir(my $__h, $__d) or return; my @__e = readdir($__h); closedir $__h; for my $__e (@__e) { last if @__out >= $__lim; next if $__e eq '.' || $__e eq '..'; my $__p = "$__d/$__e"; if (-d $__p && !-l $__p) { $__walk->($__p); } elsif (substr($__e, -3) eq '.sh') { push @__out, $__p; } } }; $__walk->('.'); splice(@__out, 3) if @__out > 3; if (@__out) { print STDOUT join("\n", @__out), "\n"; } }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;say "exit: $CHILD_ERROR";  # line 4
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
