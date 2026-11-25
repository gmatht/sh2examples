#!/usr/bin/env perl
use strict;
use warnings;
my $word = '';

open my $__fh, '>', "/tmp/084_test.txt" or die "Cannot write file: $!\n";
print {$__fh} "hello\n";
close $__fh;
$? = 0 << 8;
open my $__fh, '>>', "/tmp/084_test.txt" or die "Cannot write file: $!\n";
print {$__fh} "world\n";
close $__fh;
$? = 0 << 8;
{
    open(my $__sh2_is0, '<&', \*STDIN);
    open(STDIN, '<', "/tmp/084_test.txt") or die "Cannot read input: $!\n";
    while (do { my $__rd = <STDIN>; if (defined $__rd) { chomp $__rd; $word = $__rd; 1 } else { 0 } }) {
        do { my $__t = $word . "\n"; $__t =~ tr/a-z/A-Z/; print $__t; $? = 0 << 8; };
    }
if ($__sh2_is0) { open(STDIN, '<&', $__sh2_is0); close($__sh2_is0); }
}
do { unlink('/tmp/084_test.txt'); $? = 0 << 8; };  # line 9

exit $? >> 8;
