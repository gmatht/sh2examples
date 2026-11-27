#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $MAXWAIT;

$MAXWAIT = "20 10";
$ENV{MAXWAIT} = ${MAXWAIT} if exists $ENV{MAXWAIT};
if ((0+(($MAXWAIT =~ s{ [^ ]*$}{}r))) > (0+(($MAXWAIT =~ s{^.*? }{}r)))) {
    print "compare done\n";
    $? = 0 << 8;
}

exit $? >> 8;
