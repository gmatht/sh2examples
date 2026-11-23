#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

print "ax/pz ax/qz ayz\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "ad bd cd\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "src/main/a src/main/b src/test/a src/test/b docs/a docs/b\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "file.txt file.bak file.orig\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "prexpost preypost\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "source.bat source.c\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print("pre");  # line 18
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "xpost ypost\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "done: $CHILD_ERROR";  # line 20
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
