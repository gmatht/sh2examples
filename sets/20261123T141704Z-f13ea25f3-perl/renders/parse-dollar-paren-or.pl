#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;
my $x;
my $r = '';

$r = "hello";
if ($CHILD_ERROR != 0) {
    $x = $CHILD_ERROR;
}
say $r;  # line 10
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "x=[$x]\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
