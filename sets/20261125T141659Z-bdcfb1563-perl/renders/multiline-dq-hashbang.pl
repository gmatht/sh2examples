#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';


print "#!/bin/sh\nexec 'command' --option \"\$\@\"\n";
$? = 0 << 8;

exit $? >> 8;
