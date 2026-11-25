#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $lang;

for my $__lang (("cs", "de", "es", "fi", "fr")) {  # line 2
    $lang = $__lang;
    say $lang;
    $? = 0 << 8;
}

exit $? >> 8;
