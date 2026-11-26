#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


if (((((!(-l '/path') && -d '/path') && -f '/path/file')) || ((-l '/path' && length("$(readlink")))) && (system('bash', '-c', '"dpkg" "--compare-versions" "--" "1.0" "le" "2.0"') == 0)) {
    print "complex condition met\n";
    $? = 0 << 8;
}

exit $? >> 8;
