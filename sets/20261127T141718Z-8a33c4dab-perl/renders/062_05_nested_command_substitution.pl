#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "Testing nested command substitution...\n";
$? = 0 << 8;
system('bash', '-c', q['echo' "Current dir: ${PWD:-$(pwd)}" | 'tr' '-d' '/\\' | 'grep' '-o' '.....$']);

exit $? >> 8;
