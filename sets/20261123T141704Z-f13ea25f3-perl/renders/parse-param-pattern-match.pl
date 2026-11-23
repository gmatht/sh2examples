#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $p;
my $basename;

$p = "/opt/tool/git-tool";  # line 3
$basename = (($p =~ s{^.*[/\\]}{}r));  # line 4
say (($basename =~ s{^git-}{}r));  # line 5
$? = 0 << 8;

exit $? >> 8;
