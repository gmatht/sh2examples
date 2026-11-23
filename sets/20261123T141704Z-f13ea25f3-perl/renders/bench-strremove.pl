#!/usr/bin/env perl
use strict;
use warnings;
my $s;

$s = "abc:def:ghi";  # line 3
print("short-prefix=" . ((($s =~ s{^.*?:}{}r))), "\n");  # line 4
print("long-prefix=" . ((($s =~ s{^.*:}{}r))), "\n");  # line 5
print("short-suffix=" . ((($s =~ s{:[^:]*$}{}r))), "\n");  # line 6
print("long-suffix=" . ((($s =~ s{:.*$}{}r))), "\n");  # line 7
$? = 0 << 8;

exit $? >> 8;
