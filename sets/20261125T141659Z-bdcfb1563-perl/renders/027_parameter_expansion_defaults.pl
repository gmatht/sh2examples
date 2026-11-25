#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $maybe = '';

print "== Default values ==\n";
$? = 0 << 8;
do { { no strict 'refs'; undef ${'maybe'}; undef @{'maybe'}; undef %{'maybe'}; delete $ENV{'maybe'}; }  # line 7
$? = 0 << 8; };
say ((defined($maybe) && length($maybe)) ? $maybe : "default");  # line 8
say ((defined($maybe) && length($maybe)) ? $maybe : ($maybe = "default"));  # line 9
say $maybe;  # line 10
$? = 0 << 8;

exit $? >> 8;
