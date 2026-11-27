#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "Testing complex brace expansion...\n";
$? = 0 << 8;
print "a1x a1y a1z a2x a2y a2z a3x a3y a3z b1x b1y b1z b2x b2y b2z b3x b3y b3z c1x c1y c1z c2x c2y c2z c3x c3y c3z\n";
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 7
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
