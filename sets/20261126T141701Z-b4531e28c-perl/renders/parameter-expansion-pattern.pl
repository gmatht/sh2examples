#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $A;
my $C;

$A = (($ENV{B} =~ s{\..*$}{}r));  # line 4
$ENV{A} = ${A} if exists $ENV{A};
$C = (($ENV{D} =~ s{^.*?_}{}r));  # line 5
$ENV{C} = ${C} if exists $ENV{C};
say "$A $C";  # line 6
$? = 0 << 8;

exit $? >> 8;
