#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $val;

$val = (($val =~ s{^"}{}r));
$val = do { my $__t = $val; my $__r = $__t; for (my $__i = length($val); $__i >= 0; $__i--) { if (substr($__t, $__i) =~ /\A(?:")\z/) { $__r = substr($__t, 0, $__i); last } } $__r };
say $val;  # line 4
$? = 0 << 8;

exit $? >> 8;
