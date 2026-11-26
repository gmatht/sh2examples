#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;
use feature 'say';

my $X;

$X = "test";
$ENV{X} = ${X} if exists $ENV{X};
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "--x=$X";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
if ($CHILD_ERROR != 0) {
    $X = "0";
    $ENV{X} = ${X} if exists $ENV{X};
}

exit $? >> 8;
