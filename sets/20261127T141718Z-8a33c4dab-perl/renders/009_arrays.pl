#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my @arr;
my $x;
my %map;
my $k;
my $arr;
my $map;

print "== Indexed arrays ==\n";
$? = 0 << 8;
@arr = ("one", "two", "three");  # line 9
say $arr[1];  # line 10
say scalar(@arr);
$? = 0 << 8;
for my $__x ((@arr)) {
    $x = $__x;
    print "$x ";
    $? = 0 << 8;
}
print "\n";  # line 12
$? = 0 << 8;
print "== Associative arrays ==\n";
$? = 0 << 8;
%map = ();  # line 15
$map{'foo'} = "bar";  # line 16
$map{'answer'} = "42";  # line 17
$map{'two'} = "1 + 1";
say $map{'foo'};  # line 19
say $map{'answer'};  # line 20
my $__fs_buf = do { my $__cap = ''; open(my $__fo, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', \$__cap) or die "capture: $!\n"; my $__sh_loop_rc0 = 0;  # line 23
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
