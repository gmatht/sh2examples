#!/usr/bin/env perl
use strict;
use warnings;

print("hello world\nthis line is not terminated\n\necho \"done: $?\"\n");$? = 0 << 8;  # line 4

exit $? >> 8;
