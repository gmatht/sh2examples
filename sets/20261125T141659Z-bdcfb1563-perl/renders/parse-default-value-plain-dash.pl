#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $MYVAR;

$MYVAR = ((defined($ENV{UMASK}) && length($ENV{UMASK})) ? $ENV{UMASK} : "0640");  # line 6
$ENV{MYVAR} = ${MYVAR} if exists $ENV{MYVAR};
say $MYVAR;  # line 7
$? = 0 << 8;

exit $? >> 8;
