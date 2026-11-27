#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $count;
my $line = '';

$count = 0;  # line 4
my $__wr_buf = do { my $__cap = ''; open(my $__wo, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', \$__cap) or die "capture: $!\n"; { local *STDIN; open(STDIN, '<', '/etc/passwd') or die "read: $!\n"; my $__sh_loop_rc0 = 0;  # line 5
while (do { my $__rd = <STDIN>; if (defined $__rd) { chomp $__rd; $line = $__rd; 1 } else { 0 } }) {
    $count = int(($count + 1));
    say "$count: $line";
    $? = 0 << 8;
    $__sh_loop_rc0 = $? >> 8;
}
$? = ($__sh_loop_rc0) << 8;
 } open(STDOUT, '>&', $__wo) or die "capture: $!\n"; close($__wo); $__cap };
my @__wr = split(/\n/, $__wr_buf); @__wr = @__wr[0 .. 2] if @__wr > 3; print "$_\n" for @__wr; $? = 0 << 8;

exit $? >> 8;
