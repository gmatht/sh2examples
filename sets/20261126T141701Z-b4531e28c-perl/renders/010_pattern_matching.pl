#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;
my $__nocasematch = 0;

print "== [[ pattern and regex ]]\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
if (('file.txt' =~ /^(?s).*\.txt$/)) {
    print "pattern-match\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    }
if (('file.txt' =~ '^file\\.[a-z]+$')) {
    print "regex-match\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    }
print "== extglob ==\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 14
if ((('file.js') =~ /(?s)\.js$/) && (('file.js') !~ /(?s).*\.min\.js$/)) {
    print "f1-ok\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    }
if (!((('thing.min.js') =~ /(?s)\.js$/) && (('thing.min.js') !~ /(?s).*\.min\.js$/))) {
    print "f2-filtered\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
print "== nocasematch ==\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$__nocasematch = 1;$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 20
$CHILD_ERROR = ((($__nocasematch) ? lc('Foo') eq lc('foo') : 'Foo' eq 'foo')) ? 0 : 1;
if ((($__nocasematch) ? lc('Foo') eq lc('foo') : 'Foo' eq 'foo')) {
    print "ci-match\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    $CHILD_ERROR = 0;
}

exit $? >> 8;
