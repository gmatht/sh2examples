#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $UMASK;

$UMASK = ((defined($UMASK) && length($UMASK)) ? $UMASK : "0640");  # line 3
$ENV{UMASK} = ${UMASK} if exists $ENV{UMASK};
say $UMASK;  # line 4
$? = 0 << 8;

exit $? >> 8;
