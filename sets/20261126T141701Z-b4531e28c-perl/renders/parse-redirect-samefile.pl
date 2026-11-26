#!/usr/bin/env perl
use strict;
use warnings;

use feature 'say';

say (($ENV{PATH} =~ s{:.*$}{}r));  # line 3
$? = 0 << 8;

exit $? >> 8;
