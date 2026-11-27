#!/usr/bin/env perl
use strict;
use warnings;
my $__argc = @ARGV;
my $RESULT;

sub test {  # line 5
    local @ARGV = @_;
    my $__argc = @ARGV;
    print "hello world\n";
    $? = 0 << 8;
    $RESULT = "test";
    $ENV{RESULT} = ${RESULT} if exists $ENV{RESULT};
}
print "RESULT=[$RESULT]\n";
$? = 0 << 8;

exit $? >> 8;
