#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


my %SIG;

$SIG{INT} = sub {  # line 3
print 'Interrupted', "\n";
};
print "Trap set\n";
$? = 0 << 8;
print("exit: " . (($? >> 8)), "\n");  # line 6
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
