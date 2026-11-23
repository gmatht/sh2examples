#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $dest_root;
my $emmccheck = '';

$dest_root = ($emmccheck) . "p3";  # line 3
say join(' ', grep { defined($_) && length($_) } split(/\s+/, $dest_root));  # line 4
$? = 0 << 8;

exit $? >> 8;
