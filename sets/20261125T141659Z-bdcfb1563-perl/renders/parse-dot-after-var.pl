#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


if (1) {
    print "failed\n";
    $? = 0 << 8;
}
print("DEST=[/path/to/file]\n");
$? = 0 << 8;

exit $? >> 8;
