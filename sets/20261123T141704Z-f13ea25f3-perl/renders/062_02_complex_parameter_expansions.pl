#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $complex_var;

print "Testing complex parameter expansions...\n";
$? = 0 << 8;
$complex_var = "hello world";
say (($complex_var =~ s{^.*?o}{}r));  # line 6
say (($complex_var =~ s{^.*o}{}r));  # line 7
say (($complex_var =~ s{o[^o]*$}{}r));  # line 8
say (($complex_var =~ s{o.*$}{}r));  # line 9
$? = 0 << 8;

exit $? >> 8;
