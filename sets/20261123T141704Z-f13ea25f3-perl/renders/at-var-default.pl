#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $OPTS;

$OPTS = ((defined(join(' ', @ARGV)) && length(join(' ', @ARGV))) ? join(' ', @ARGV) : "default");  # line 3
$ENV{OPTS} = ${OPTS} if exists $ENV{OPTS};
say $OPTS;  # line 4
$? = 0 << 8;

exit $? >> 8;
