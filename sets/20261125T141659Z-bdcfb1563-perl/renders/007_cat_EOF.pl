#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

print("alpha\nbeta\ngamma ...\n");
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print("oyster\nsnapper\nsalmon\n");
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "Fin. That is all folks.\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "exit: $CHILD_ERROR";  # line 15
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
