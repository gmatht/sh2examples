#!/usr/bin/env perl
use strict;
use warnings;
my $VAR;
my $VAR2;

$VAR = "hello";  # line 3
$ENV{VAR} = ${VAR} if exists $ENV{VAR};
$VAR2 = "world";  # line 4
$ENV{VAR2} = ${VAR2} if exists $ENV{VAR2};
print("$VAR more $VAR2 text\n");$? = 0 << 8;
print("VAR=[hello]\n");
$? = 0 << 8;
print("VAR2=[world]\n");
$? = 0 << 8;

exit $? >> 8;
