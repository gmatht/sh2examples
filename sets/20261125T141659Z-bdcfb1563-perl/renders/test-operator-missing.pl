#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


if (-l '/dev/null') {  # line 3
    } else {
    }
if (-p '/dev/null') {  # line 4
    } else {
    }
if (-b '/dev/null') {  # line 5
    } else {
    }
if (-c '/dev/null') {  # line 6
    } else {
    }
if (-g '/dev/null') {  # line 7
    } else {
    }
if (-k '/dev/null') {  # line 8
    } else {
    }
if (-u '/dev/null') {  # line 9
    } else {
    }
if (length('-O')) {  # line 10
    } else {
    }
if (length('-G')) {  # line 11
    } else {
    }
if (length('-N')) {  # line 12
    } else {
    }
if ((-e 'a' && -e 'b' && (stat('a'))[9] > (stat('b'))[9])) {  # line 14
    } else {
    }
if ((-e 'a' && -e 'b' && (stat('a'))[9] < (stat('b'))[9])) {  # line 15
    } else {
    }
if ((-e 'a' && -e 'b' && (stat('a'))[0] == (stat('b'))[0] && (stat('a'))[1] == (stat('b'))[1])) {  # line 16
    } else {
    }
if (1) {
    }
if (1) {
    $? = 0 << 8;
}
print("parsed OK\n");
$? = 0 << 8;
say "status: ok";
$? = 0 << 8;

exit $? >> 8;
