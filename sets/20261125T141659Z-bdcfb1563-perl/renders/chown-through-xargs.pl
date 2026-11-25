#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


do { my $__it = '/tmp/chown_through_xargs_test' . "\n"; if (length($__it)) { print STDOUT join(' ', 'chown', '-c', 'root:root', '--', '/tmp/chown_through_xargs_test'), " ", $__it, "\n"; } }; $? = 0 << 8;say "status: ok";
$? = 0 << 8;

exit $? >> 8;
