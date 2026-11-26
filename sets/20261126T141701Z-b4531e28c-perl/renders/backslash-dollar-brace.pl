#!/usr/bin/env perl
use strict;
use warnings;
my $var;

for my $__var (("BASH_ENV", "ENV", "MAIL")) {  # line 4
    $var = $__var;
    $ENV{var} = $var;
    system('bash', '-c', q('eval' 'test' "\\${$var+y}" && 'unset' "$var" || ':'));
}
print("parsed OK\n");
$? = 0 << 8;

exit $? >> 8;
