#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


system('bash', '-c', q{( echo 'test' ) <<_SH2DOC_ 2>&1 > '/dev/null'
first line
second line
_SH2DOC_});  # line 4
print "after\n";
$? = 0 << 8;
print("done: " . (($? >> 8)), "\n");  # line 10
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
