#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;
my $file = '';

$ENV{file} = $file;  # line 5
system('bash', '-c', q[( ( 'gzip' '-cdfq' '--' "${file}" 4> '-'; 'echo' "$CHILD_ERROR" 1>&4 ) 3> '-' | 'cat' ) 5<&0]);$CHILD_ERROR = $? >> 8;

exit $? >> 8;
