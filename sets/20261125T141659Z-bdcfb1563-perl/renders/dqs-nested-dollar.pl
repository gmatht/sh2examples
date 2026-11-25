#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $name;

$name = do { my $__cap = do { my $__t = $ENV{USER} . "\n"; $__t =~ tr/A-Z/a-z/; $__t }; $__cap =~ s/\n+\z//; $__cap };  # line 4
say $name;  # line 5
$? = 0 << 8;

exit $? >> 8;
