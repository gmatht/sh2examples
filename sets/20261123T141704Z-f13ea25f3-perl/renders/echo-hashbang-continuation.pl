#!/usr/bin/env perl
use strict;
use warnings;

system('bash', '-c', q['echo' "#!/bin/sh
exec 'do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     print "test\n";
    $? = 0 << 8;
;; } $__cap =~ s/\n+\z//; $__cap }' --flag \"\$@\"" > '/tmp/echo_hashbang_continuation_output.sh']);  # line 1

exit $? >> 8;
