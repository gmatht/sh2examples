#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

sub strHasPrefix {
    return index($_[0], $_[1]) == 0;
}
sub strHasSuffix {
    my ($__h, $__n) = @_;
    return length($__n) <= length($__h) && substr($__h, length($__h) - length($__n)) eq $__n;
}

$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 4
print "== [[ pattern and regex ]]\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = ((strHasSuffix("file.txt", ".txt")) ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;
if (strHasSuffix("file.txt", ".txt")) {
    print "pattern-match\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
$CHILD_ERROR = (('file.txt' =~ '^file\\.[a-z]+$')) ? 0 : 1;
if (('file.txt' =~ '^file\\.[a-z]+$')) {
    print "regex-match\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    $CHILD_ERROR = 0;
}

exit $? >> 8;
