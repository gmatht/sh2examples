#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $p;
my $result;

$p = "/home/user/script";  # line 4
$result = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     say join(' ', grep { defined($_) && length($_) } split(/\s+/, (($p =~ s{^/}{}r))));  # line 5
    $? = 0 << 8;
;; } $__cap =~ s/\n+\z//; $__cap };
say $result;  # line 6
$? = 0 << 8;

exit $? >> 8;
