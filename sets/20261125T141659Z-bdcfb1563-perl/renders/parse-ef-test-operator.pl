#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


if ((system('bash', '-c', q('test' "${A}" '-ef' "${B}" 2> '/dev/null')) == 0)) {
    print "same file\n";
    $? = 0 << 8;
}

exit $? >> 8;
