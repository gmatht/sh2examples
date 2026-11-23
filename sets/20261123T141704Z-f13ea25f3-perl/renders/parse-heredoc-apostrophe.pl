#!/usr/bin/env perl
use strict;
use warnings;
my $result;

$result = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     print("It isn't broken\n");  # line 6
    $? = 0 << 8;
    $? = 0 << 8;
;; } $__cap =~ s/\n+\z//; $__cap };
print "heredoc content=[$result]\n";  # line 10
$? = 0 << 8;

exit $? >> 8;
