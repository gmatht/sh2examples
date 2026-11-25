#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $__sh2_last_bg = '';

say "background: $__sh2_last_bg";  # line 3
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
