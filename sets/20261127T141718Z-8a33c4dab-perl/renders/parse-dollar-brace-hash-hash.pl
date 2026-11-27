#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;
my $__nocasematch = 0;
my $p;

$p = "/usr/local/bin/example";  # line 5
$CHILD_ERROR = ((($__nocasematch) ? lc((($p =~ s{^.*/}{}r))) eq lc('example') : (($p =~ s{^.*/}{}r)) eq 'example')) ? 0 : 1;
if ((($__nocasematch) ? lc((($p =~ s{^.*/}{}r))) eq lc('example') : (($p =~ s{^.*/}{}r)) eq 'example')) {
    print "it is example\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    $CHILD_ERROR = 0;
}

exit $? >> 8;
