#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $opts;

$opts = ((defined(join(' ', @ARGV)) && length(join(' ', @ARGV))) ? join(' ', @ARGV) : "--help");  # line 3
say $opts;  # line 4
$? = 0 << 8;

exit $? >> 8;
