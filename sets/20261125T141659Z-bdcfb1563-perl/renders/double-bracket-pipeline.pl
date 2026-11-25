#!/usr/bin/env perl
use strict;
use warnings;
my $f;

$f = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 4
$? = (($f ne "" && -f $f) ? 0 : 1) << 8;
if ($f ne "" && -f $f) {
    print "File exists\n";
    $? = 0 << 8;
}
do { unlink($f); $? = 0 << 8; };  # line 6

exit $? >> 8;
