#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $tsv;
my $i;

$tsv = "";
for my $__i (("1", "2", "3", "4", "5")) {  # line 4
    $i = $__i;
    $tsv = ($tsv) . ($i) . ",";
}
say $tsv;  # line 7
$? = 0 << 8;

exit $? >> 8;
