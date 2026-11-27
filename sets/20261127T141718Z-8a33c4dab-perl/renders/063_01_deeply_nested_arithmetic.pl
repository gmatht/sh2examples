#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $a;
my $b;
my $c;
my $d;
my $e;
my $f;
my $g;
my $h;
my $i;
my $j;
my $k;
my $l;
my $m;
my $n;
my $result;

$a = "10";  # line 4
$b = "3";
$c = "8";
$d = "2";
$e = "7";
$f = "2";
$g = "2";
$h = "3";
$i = "4";
$j = "1";
$k = "6";
$l = "3";
$m = "5";
$n = "2";
$result = int(((0+(((do { my $__d = (do { my $__d = $f; eval { int(($e)) % ($__d) } }); eval { int(((($a + $b) * ($c - $d))) / ($__d)) } }) + ($g ** $h)) - ((0+$i) << (0+$j)))) | (0+((0+((0+$k) & (0+$l))) ^ (0+((0+$m) | (0+$n)))))));  # line 5
say "Deeply nested arithmetic result: $result";  # line 6
$? = 0 << 8;

exit $? >> 8;
