#!/usr/bin/env perl
use strict;
use warnings;

use feature 'say';

say join(' ', grep { defined($_) && length($_) } split(/\s+/, ((defined($ENV{MOUNTPOINT}) && length($ENV{MOUNTPOINT})) ? $ENV{MOUNTPOINT} : $ENV{NAME})));  # line 4
$? = 0 << 8;

exit $? >> 8;
