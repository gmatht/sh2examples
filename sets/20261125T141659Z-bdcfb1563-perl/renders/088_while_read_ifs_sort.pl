#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $name = '';
my $num = '';

open my $__fh, '>', "/tmp/088_data.txt" or die "Cannot write file: $!\n";
print {$__fh} "b:2\n";
close $__fh;
$? = 0 << 8;
open my $__fh, '>>', "/tmp/088_data.txt" or die "Cannot write file: $!\n";
print {$__fh} "a:1\n";
close $__fh;
$? = 0 << 8;
open my $__fh, '>>', "/tmp/088_data.txt" or die "Cannot write file: $!\n";
print {$__fh} "c:3\n";
close $__fh;
$? = 0 << 8;
my $__wr_buf = do { my $__cap = ''; open(my $__wo, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', \$__cap) or die "capture: $!\n"; { local *STDIN; open(STDIN, '<', '/tmp/088_data.txt') or die "read: $!\n"; while (do { my $__rd = <STDIN>; if (defined $__rd) { chomp $__rd; ($name, $num) = split(/[:]/, $__rd, 2); 1 } else { 0 } }) {  # line 7
    say "$num $name";
    $? = 0 << 8;
}
 } open(STDOUT, '>&', $__wo) or die "capture: $!\n"; close($__wo); $__cap };
my @__wr = split(/\n/, $__wr_buf); @__wr = sort { our ($a, $b); $a <=> $b } @__wr; print "$_\n" for @__wr; $? = 0 << 8;
do { unlink('/tmp/088_data.txt'); $? = 0 << 8; };  # line 10

exit $? >> 8;
