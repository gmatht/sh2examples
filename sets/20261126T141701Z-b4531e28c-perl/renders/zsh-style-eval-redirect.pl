#!/usr/bin/env perl
use strict;
use warnings;
my $__argc = @ARGV;

sub proxy {  # line 2
    local @ARGV = @_;
    my $__argc = @ARGV;
    if ((length($ENV{DEFAULT_PROXY}) && !length($ENV{SHELLPROXY_URL}))) {
        $ENV{SHELLPROXY_URL} = $ENV{DEFAULT_PROXY};
    }
}
print("SHELLPROXY_URL" . "=[" . ($ENV{SHELLPROXY_URL}) . "]\n");
$? = 0 << 8;

exit $? >> 8;
