#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $a;

$a = "1";  # line 1
say join(' ', grep { defined($_) && length($_) } split(/\s+/, $a));  # line 2
{  # line 3
    local %ENV = %ENV;
    my $__save_a = $a;
    $a = "2";
    say join(' ', grep { defined($_) && length($_) } split(/\s+/, $a));
        $a = $__save_a;
}
{  # line 4
    local %ENV = %ENV;
    say join(' ', grep { defined($_) && length($_) } split(/\s+/, $a));
    }
say join(' ', grep { defined($_) && length($_) } split(/\s+/, $a));  # line 5
$? = 0 << 8;

exit $? >> 8;
