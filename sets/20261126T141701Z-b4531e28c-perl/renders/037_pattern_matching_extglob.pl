#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "== extglob ==\n";
$? = 0 << 8;
$? = 0 << 8;  # line 7
if ((('file.js') =~ /(?s)\.js$/) && (('file.js') !~ /(?s).*\.min\.js$/)) {
    print "f1-ok\n";
    $? = 0 << 8;
} else {
    }
if (!((('thing.min.js') =~ /(?s)\.js$/) && (('thing.min.js') !~ /(?s).*\.min\.js$/))) {
    print "f2-filtered\n";
    $? = 0 << 8;
}

exit $? >> 8;
