#!/usr/bin/env perl
use strict;
use warnings;

my $d;

$ENV{d} = $d;  # line 2
system('bash', '-c', q['readarray' '-t' 'files' <<< $(for d in "$@"; do echo "${d}"; done)]);

exit $? >> 8;
