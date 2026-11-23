#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my @FilesystemOptions;
my $FilesystemOptions;

@FilesystemOptions = ("a", "b", "c");  # line 2
say $FilesystemOptions[(2*1)-1];
$? = 0 << 8;

exit $? >> 8;
