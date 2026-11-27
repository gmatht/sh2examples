#!/usr/bin/env perl
use strict;
use warnings;
my $__argc = @ARGV;

sub g {  # line 28
    local @ARGV = @_;
    my $__argc = @ARGV;
    open(my $__sh2_rfh0, '>&', \*STDERR) or die "Cannot dup stderr: $!\n";
    close(STDERR); open(STDERR, '>', "/dev/null") or die "Cannot write to /dev/null: $!\n";
    print "hi\n";
    $? = 0 << 8;
    open(STDERR, '>&', $__sh2_rfh0) or die "Cannot restore stderr: $!\n"; close($__sh2_rfh0);
}
sub f {  # line 29
    local @ARGV = @_;
    my $__argc = @ARGV;
    g();
}
f();  # line 30

exit $? >> 8;
