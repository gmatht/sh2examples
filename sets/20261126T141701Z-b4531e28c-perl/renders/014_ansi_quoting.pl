#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


$? = 0 << 8;  # line 6
print "== ANSI-C quoting ==\n";
$? = 0 << 8;
print "line1\nline2\tTabbed\n";  # line 9
$? = 0 << 8;
print "== Escape sequences ==\n";
$? = 0 << 8;
print "bell\n";  # line 12
$? = 0 << 8;
print "backspace\n";  # line 13
$? = 0 << 8;
print "formfeed\n";  # line 14
$? = 0 << 8;
print "newline\n\n";  # line 15
$? = 0 << 8;
print "carriage\rreturn\n";  # line 16
$? = 0 << 8;
print "tab\tseparated\n";  # line 17
$? = 0 << 8;
print "verticaltab\n";  # line 18
$? = 0 << 8;
print "== Unicode and hex ==\n";
$? = 0 << 8;
print "Hello\n";  # line 21
$? = 0 << 8;
print "Hello\n";  # line 22
$? = 0 << 8;
print "== Practical examples ==\n";
$? = 0 << 8;
print("Name       Age        City\n");
$? = 0 << 8;
print("John       25         NYC\n");
$? = 0 << 8;
print("Jane       30         LA\n");
$? = 0 << 8;

exit $? >> 8;
