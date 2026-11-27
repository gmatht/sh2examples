#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $maybe = '';

print "== Case modification in parameter expansion ==\n";
$? = 0 << 8;
print "WORLD\n";
$? = 0 << 8;
print "world\n";
$? = 0 << 8;
print "World\n";
$? = 0 << 8;
print "== Advanced parameter expansion ==\n";
$? = 0 << 8;
print "013_param_expansion_file.txt\n";
$? = 0 << 8;
print "/tmp\n";
$? = 0 << 8;
print "aXXa\n";
$? = 0 << 8;
print "== More parameter expansion ==\n";
$? = 0 << 8;
print " world\n";
$? = 0 << 8;
print "hello \n";
$? = 0 << 8;
print "hell0 w0rld\n";
$? = 0 << 8;
print "== Default values ==\n";
$? = 0 << 8;
do { { no strict 'refs'; undef ${'maybe'}; undef @{'maybe'}; undef %{'maybe'}; delete $ENV{'maybe'}; }  # line 27
$? = 0 << 8; };
say ((defined($maybe) && length($maybe)) ? $maybe : "default");  # line 28
say ((defined($maybe) && length($maybe)) ? $maybe : ($maybe = "default"));  # line 29
say $maybe;  # line 30
$? = 0 << 8;

exit $? >> 8;
