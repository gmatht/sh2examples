#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


my $PWD;
my $USER;

open my $__fh, '>', "config.txt" or die "Cannot write file: $!\n";  # line 4
print {$__fh} "User: \$USER\nHost: \${HOSTNAME:-localhost}\nPath: \$PWD\n";
close $__fh;
$? = 0 << 8;
print "Config file created\n";
$? = 0 << 8;

exit $? >> 8;
