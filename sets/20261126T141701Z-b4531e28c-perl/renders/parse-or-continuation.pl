#!/usr/bin/env perl
use strict;
use warnings;
my $__argc = @ARGV;

sub die {  # line 3
    local @ARGV = @_;
    my $__argc = @ARGV;
    system('bash', '-c', 'echo "${*}" 1>&2');
    exit "1";
}
chdir("/tmp") or do { die("Could not cd");$? = 0 << 8; };

exit $? >> 8;
