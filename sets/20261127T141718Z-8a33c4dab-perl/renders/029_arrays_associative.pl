#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my %map;
my $k;
my $map;

print "== Associative arrays ==\n";
$? = 0 << 8;
%map = ();  # line 7
$map{'foo'} = "bar";  # line 8
$map{'answer'} = "42";  # line 9
$map{'two'} = "1 + 1";
say $map{'foo'};  # line 11
say $map{'answer'};  # line 12
my $__fs_buf = do { my $__cap = ''; open(my $__fo, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', \$__cap) or die "capture: $!\n"; my $__sh_loop_rc0 = 0;  # line 15
for my $__k ((keys %map)) {
    $k = $__k;
    print(($k) . " => " . ($map{$k}), "\n");
    $? = 0 << 8;
    $__sh_loop_rc0 = $? >> 8;
}
$? = ($__sh_loop_rc0) << 8;
 open(STDOUT, '>&', $__fo) or die "capture: $!\n"; close($__fo); $__cap };
my @__fs = split(/\n/, $__fs_buf); @__fs = sort { our ($a, $b); $a cmp $b } @__fs; print "$_\n" for @__fs; $? = 0 << 8;

exit $? >> 8;
