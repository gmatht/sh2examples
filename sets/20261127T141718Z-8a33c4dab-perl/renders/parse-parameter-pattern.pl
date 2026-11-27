#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $path;

$path = "/usr/local/bin";
say (($path =~ s{/.*$}{}r));  # line 3
say (($path =~ s{^.*?/}{}r));  # line 4
$? = 0 << 8;

exit $? >> 8;
