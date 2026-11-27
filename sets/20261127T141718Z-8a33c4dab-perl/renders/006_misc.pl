#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "== Subshell ==\n";
$? = 0 << 8;
{  # line 4
    local %ENV = %ENV;
    print "inside-subshell\n";
    $? = 0 << 8;
    }
print "== Simple pipeline ==\n";
$? = 0 << 8;
do { my $__g = 'alpha beta'; my $__m = 0; for my $__l (split(/\n/, $__g)) { if ($__l =~ /beta/) { print $__l, "\n"; $__m = 1; } } $? = ($__m ? 0 : 1) << 8; };
print("exit: " . (($? >> 8)), "\n");  # line 11
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
