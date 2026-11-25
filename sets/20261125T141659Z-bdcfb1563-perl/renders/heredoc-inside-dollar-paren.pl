#!/usr/bin/env perl
use strict;
use warnings;
my $x;

$x = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     print("line1\nline2\n");  # line 3
    $? = 0 << 8;
    $? = 0 << 8;
;; } $__cap =~ s/\n+\z//; $__cap };
print "result=[$x]\n";  # line 8
$? = 0 << 8;

exit $? >> 8;
