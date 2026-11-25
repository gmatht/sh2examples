#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

system('bash', '-c', q{( echo 'test' ) <<_SH2DOC_ 2>&1 > '/dev/null'
first line
second line
_SH2DOC_});$CHILD_ERROR = $? >> 8;  # line 4
print "after\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "done: $CHILD_ERROR";  # line 10
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
