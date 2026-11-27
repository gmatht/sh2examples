#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;

use feature 'say';

$CHILD_ERROR = ((-e '/path' && -e '/other' && (stat('/path'))[0] == (stat('/other'))[0] && (stat('/path'))[1] == (stat('/other'))[1])) ? 0 : 1;
if ((-e '/path' && -e '/other' && (stat('/path'))[0] == (stat('/other'))[0] && (stat('/path'))[1] == (stat('/other'))[1])) {
    print "same\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
$? = 1 << 8; $CHILD_ERROR = $? >> 8;
}
say "exit: $CHILD_ERROR";  # line 4
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
