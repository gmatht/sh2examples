#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 3
$CHILD_ERROR = (('foo' =~ /^(?s)(?:foo|bar)$/)) ? 0 : 1;
if (('foo' =~ /^(?s)(?:foo|bar)$/)) {
    print "matched\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    $CHILD_ERROR = 0;
}
print("parsed OK\n");
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
