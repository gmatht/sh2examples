#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $tsv;
my $i;

$tsv = "";
my $__sh_loop_rc0 = 0;  # line 4
for my $__i (("1", "2", "3", "4", "5")) {
    $i = $__i;
    $tsv = ($tsv) . ($i) . ",";
    $__sh_loop_rc0 = $? >> 8;
}
$? = ($__sh_loop_rc0) << 8;
say $tsv;  # line 7
$? = 0 << 8;

exit $? >> 8;
