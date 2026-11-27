#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print("alpha\nbeta\ngamma ...\n");
$? = 0 << 8;
print("oyster\nsnapper\nsalmon\n");
$? = 0 << 8;
print "Fin. That is all folks.\n";
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 15
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
