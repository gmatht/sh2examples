#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $X;

$X = "test";
$ENV{X} = ${X} if exists $ENV{X};
$? = 0 << 8;
say "--x=$X";
$? = 0 << 8;
if (($? >> 8) != 0) {
    $X = "0";
    $ENV{X} = ${X} if exists $ENV{X};
}

exit $? >> 8;
