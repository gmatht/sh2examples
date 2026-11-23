#!/usr/bin/env perl
use strict;
use warnings;
my $result;

$result = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     print("This is a test with an apostrophe: it's fine\n");  # line 3
    $? = 0 << 8;
    $? = 0 << 8;
;; } $__cap =~ s/\n+\z//; $__cap };
print "heredoc content=[$result]\n";  # line 7
$? = 0 << 8;

exit $? >> 8;
