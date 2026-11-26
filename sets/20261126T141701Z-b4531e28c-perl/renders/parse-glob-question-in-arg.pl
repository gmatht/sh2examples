#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;

$CHILD_ERROR = (-d '/var/lib/foo') ? 0 : 1;
if (-d '/var/lib/foo') {
    do { unlink(("/var/lib/foo") . "/??\\:??\\:??"); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
} else {
    $CHILD_ERROR = 0;
}
print("f=[/var/lib/foo]\n");
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
