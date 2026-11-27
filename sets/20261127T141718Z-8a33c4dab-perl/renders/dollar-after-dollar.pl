#!/usr/bin/env perl
use strict;
use warnings;

use feature 'say';

say join(' ', grep { defined($_) && length($_) } split(/\s+/, ($? >> 8)));  # line 4
say join(' ', @ARGV);  # line 5
$? = 0 << 8;
say join(' ', @ARGV);  # line 6
$? = 0 << 8;
$? = 0 << 8;

exit $? >> 8;
