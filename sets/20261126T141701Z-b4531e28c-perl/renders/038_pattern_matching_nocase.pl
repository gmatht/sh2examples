#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;
my $__nocasematch = 0;

print "== nocasematch ==\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$__nocasematch = 1;$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 7
$CHILD_ERROR = ((($__nocasematch) ? lc('Foo') eq lc('foo') : 'Foo' eq 'foo')) ? 0 : 1;
if ((($__nocasematch) ? lc('Foo') eq lc('foo') : 'Foo' eq 'foo')) {
    print "ci-match\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    $CHILD_ERROR = 0;
}

exit $? >> 8;
