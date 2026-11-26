#!/usr/bin/env perl
use strict;
use warnings;

do { unless (-f '/usr/sbin/dkms') { exit 0; } }; $? = 0 << 8;print("prog=[dkms]\n");
$? = 0 << 8;

exit $? >> 8;
