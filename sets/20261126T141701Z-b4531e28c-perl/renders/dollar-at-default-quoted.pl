#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $ARGS;

$ARGS = ((defined(join(' ', @ARGV)) && length(join(' ', @ARGV))) ? join(' ', @ARGV) : "default");  # line 4
$ENV{ARGS} = ${ARGS} if exists $ENV{ARGS};
say $ARGS;  # line 5
$? = 0 << 8;

exit $? >> 8;
