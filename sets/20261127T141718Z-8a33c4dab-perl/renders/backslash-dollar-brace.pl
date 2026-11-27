#!/usr/bin/env perl
use strict;
use warnings;
my $var;

my $__sh_loop_rc0 = 0;  # line 4
for my $__var (("BASH_ENV", "ENV", "MAIL")) {
    $var = $__var;
    $ENV{var} = $var;
    system('bash', '-c', q('eval' 'test' "\\${$var+y}" && 'unset' "$var" || ':'));
    $__sh_loop_rc0 = $? >> 8;
}
$? = ($__sh_loop_rc0) << 8;
print("parsed OK\n");
$? = 0 << 8;

exit $? >> 8;
