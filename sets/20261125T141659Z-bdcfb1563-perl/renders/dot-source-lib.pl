#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $shared_val;

open my $__fh, '>', "lib_tmp.sh" or die "Cannot write file: $!\n";  # line 3
print {$__fh} "shared_val=secret\n";
close $__fh;
$? = 0 << 8;
$shared_val = "secret";  # line 4
say "shared=$shared_val";  # line 5
$? = 0 << 8;
do { unlink('lib_tmp.sh'); $? = 0 << 8; };  # line 6

exit $? >> 8;
