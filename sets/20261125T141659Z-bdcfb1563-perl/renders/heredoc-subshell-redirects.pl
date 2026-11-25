#!/usr/bin/env perl
use strict;
use warnings;
my $var;

$var = "test";  # line 3
$? = 0 << 8;print("heredoc+subshell+redirect ran OK\n");  # line 7
$? = 0 << 8;
print "var=[$var]\n";
$? = 0 << 8;

exit $? >> 8;
