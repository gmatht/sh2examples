#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "hello\n";
$? = 0 << 8;
warn 'syntax error near unexpected token `)\'';
exit 2;

exit $? >> 8;
