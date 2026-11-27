#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


$? = 0 << 8;END {  # line 5
print 'Cleaning up...', "\n";
unlink glob('/tmp/064_23_temp_*');
}
print "Traps set up\n";
$? = 0 << 8;

exit $? >> 8;
