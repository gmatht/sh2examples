#!/usr/bin/env perl
use strict;
use warnings;

{  # line 4
    local %ENV = %ENV;
    $ENV{DEBUG} = "1";
        $ENV{LOG_LEVEL} = "verbose";
        {
        local %ENV = %ENV;
        do { { no strict 'refs'; undef ${'DEBUG'}; undef @{'DEBUG'}; undef %{'DEBUG'}; delete $ENV{'DEBUG'}; }
$? = 0 << 8; };
        print("Inner: LOG_LEVEL=" . ($ENV{LOG_LEVEL}) . ", DEBUG=" . (((defined($ENV{DEBUG}) && length($ENV{DEBUG})) ? $ENV{DEBUG} : "unset")), "\n");
            }
    print("Outer: LOG_LEVEL=" . ($ENV{LOG_LEVEL}) . ", DEBUG=" . ($ENV{DEBUG}), "\n");
    $? = 0 << 8;
    $? = 0 << 8;
}

exit $? >> 8;
