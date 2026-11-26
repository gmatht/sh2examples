#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 4
print "== Advanced brace expansion ==\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "a1 a2 a3 b1 b2 b3 c1 c2 c3\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "1 3 5 7 9\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "a d g j m p s v y\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "exit: $CHILD_ERROR";  # line 11
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
