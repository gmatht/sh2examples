#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $line = '';

open my $__fh, '>', "/tmp/while_test.txt" or die "Cannot write file: $!\n";  # line 3
print {$__fh} "a\n";
close $__fh;
$? = 0 << 8;
open my $__fh, '>>', "/tmp/while_test.txt" or die "Cannot write file: $!\n";  # line 4
print {$__fh} "b\n";
close $__fh;
$? = 0 << 8;
{  # line 5
    open(my $__sh2_is0, '<&', \*STDIN);
    open(STDIN, '<', "/tmp/while_test.txt") or die "Cannot read input: $!\n";
    while (do { my $__rd = <STDIN>; if (defined $__rd) { chomp $__rd; $line = $__rd; 1 } else { 0 } }) {
        say "Line: $line";
        $? = 0 << 8;
    }
if ($__sh2_is0) { open(STDIN, '<&', $__sh2_is0); close($__sh2_is0); }
}
do { unlink('/tmp/while_test.txt'); $? = 0 << 8; };  # line 8

exit $? >> 8;
