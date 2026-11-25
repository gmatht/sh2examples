#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;
my $__nocasematch = 0;

if ((($__nocasematch) ? lc('abc') eq lc('a*') : 'abc' eq 'a*')) {  # line 6
    print "bad\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    print "literal-star-nomatch\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
if ((($__nocasematch) ? lc('xyz') eq lc('a*') : 'xyz' eq 'a*')) {  # line 7
    print "bad\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    print "const-star-nomatch\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
if ((($__nocasematch) ? lc('abc') eq lc('a*') : 'abc' eq 'a*')) {
    print "bad\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    print "var-star-nomatch\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
if ((($__nocasematch) ? lc('abc') eq lc('a*') : 'abc' eq 'a*')) {  # line 11
    print "bad\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    print "db-bracket-nomatch\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
$CHILD_ERROR = ((($__nocasematch) ? lc('abc') eq lc('[ab]*') : 'abc' eq '[ab]*')) ? 0 : 1;  # line 13
if ((($__nocasematch) ? lc('abc') eq lc('[ab]*') : 'abc' eq '[ab]*')) {
    print "bad\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    print "bracket-literal-nomatch\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}

exit $? >> 8;
