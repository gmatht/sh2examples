#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


$? = 0 << 8;  # line 4
print "== Escape sequences ==\n";
$? = 0 << 8;
print "bell\n";  # line 7
$? = 0 << 8;
print "backspace\n";  # line 8
$? = 0 << 8;
print "formfeed\n";  # line 9
$? = 0 << 8;
print "newline\n\n";  # line 10
$? = 0 << 8;
print "carriage\rreturn\n";  # line 11
$? = 0 << 8;
print "tab\tseparated\n";  # line 12
$? = 0 << 8;
print "verticaltab\n";  # line 13
$? = 0 << 8;

exit $? >> 8;
