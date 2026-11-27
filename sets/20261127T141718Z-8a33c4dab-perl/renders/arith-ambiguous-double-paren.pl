#!/usr/bin/env perl
use strict;
use warnings;
my $file = '';

$ENV{file} = $file;  # line 5
system('bash', '-c', q[( ( 'gzip' '-cdfq' '--' "${file}" 4> '-'; 'echo' "($? >> 8)" 1>&4 ) 3> '-' | 'cat' ) 5<&0]);

exit $? >> 8;
