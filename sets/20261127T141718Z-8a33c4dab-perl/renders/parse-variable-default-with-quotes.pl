#!/usr/bin/env perl
use strict;
use warnings;

use feature 'say';

say ((defined($ENV{MYVAR}) && length($ENV{MYVAR})) ? $ENV{MYVAR} : "default value");  # line 3
$? = 0 << 8;

exit $? >> 8;
