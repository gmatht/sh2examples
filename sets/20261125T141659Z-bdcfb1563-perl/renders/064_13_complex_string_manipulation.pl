#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $filename;
my $basename;
my $extension;

$filename = "my_file.txt";
$basename = (($filename =~ s{\.[^.]*$}{}r));  # line 5
$extension = (($filename =~ s{^.*\.}{}r));  # line 6
say "Basename: $basename";
$? = 0 << 8;
say "Extension: $extension";
$? = 0 << 8;
say "Uppercase: MY_FILE.TXT";  # line 10
$? = 0 << 8;
say "Lowercase: my_file.txt";  # line 11
$? = 0 << 8;

exit $? >> 8;
