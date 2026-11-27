#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $lang;

my $__sh_loop_rc0 = 0;  # line 2
for my $__lang (("cs", "de", "es", "fi", "fr")) {
    $lang = $__lang;
    say $lang;
    $? = 0 << 8;
    $__sh_loop_rc0 = $? >> 8;
}
$? = ($__sh_loop_rc0) << 8;

exit $? >> 8;
