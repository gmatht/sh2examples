#!/usr/bin/env perl
use strict;
use warnings;

chdir("/nonexistent/path") or do { warn "cd: " . "/nonexistent/path" . ": $!\n"; print "fallback\n";
$? = 0 << 8; };
print("exit: " . (($? >> 8)), "\n");  # line 6
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
