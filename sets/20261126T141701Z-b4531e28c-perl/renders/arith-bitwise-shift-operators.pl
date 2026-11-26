#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


say int(((0+(5 + 3)) & (0+12)));  # line 3
$? = 0 << 8;
say int(((0+1) << (0+4)));  # line 4
$? = 0 << 8;
say int(((0+32) >> (0+2)));  # line 5
$? = 0 << 8;
say int(((0+7) | (0+8)));  # line 6
$? = 0 << 8;
say int(((0+12) ^ (0+5)));  # line 7
$? = 0 << 8;
say int(-(0) - 1);  # line 8
$? = 0 << 8;

exit $? >> 8;
