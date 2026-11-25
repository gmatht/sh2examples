#!/usr/bin/env perl
use strict;
use warnings;

my $var;

$ENV{var} = $var;  # line 4
system('bash', '-c', q|'tr' '[:upper:]' '[:lower:]' <<< $('echo' "UPPER: ${var^^}")|);

exit $? >> 8;
