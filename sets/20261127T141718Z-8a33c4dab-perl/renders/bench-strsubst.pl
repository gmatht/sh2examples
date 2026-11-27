#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


say "all=abc,def,ghi";
$? = 0 << 8;
say "front=XYZ:def:ghi";
$? = 0 << 8;
say "back=abc:def:XYZ";
$? = 0 << 8;

exit $? >> 8;
