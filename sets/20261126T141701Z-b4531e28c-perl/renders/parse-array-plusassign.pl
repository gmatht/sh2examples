#!/usr/bin/env perl
use strict;
use warnings;
my @LIST;

@LIST = ();  # line 2
push @LIST, ("item1", "item2");
print("LIST" . "=[" . (((defined($LIST[0]) && length($LIST[0])) ? $LIST[0] : "")) . "]\n");
$? = 0 << 8;

exit $? >> 8;
