#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $file;

$file = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 6
if (1) {
    print "var is set\n";
    $? = 0 << 8;
}
if (-f $file) {
    print "file exists\n";
    $? = 0 << 8;
} else {
    }
do { unlink($file); $? = 0 << 8; };  # line 11

exit $? >> 8;
