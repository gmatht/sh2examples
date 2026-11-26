#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $d;
my $tee_result;

print "=== Output and Formatting Commands ===\n";
$? = 0 << 8;
say "Echo result: Hello from backticks";
$? = 0 << 8;
$d = do { my $__sh_cap; $__sh_cap = do { require File::Temp; File::Temp::tempdir('sh2_ps_XXXXXX', TMPDIR => 1, CLEANUP => 0) }; $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 15
chdir($d) or do { exit "1"; };  # line 16
say "Printf result: Number: 42, String: test";
$? = 0 << 8;
$tee_result = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); do { open(my $__t, '>', 'test_tee.txt') or die "tee: 'test_tee.txt': $!\n"; print {$__t} 'test output', "\n"; close($__t); print 'test output', "\n"; $? = 0 << 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
say "Tee result: $tee_result";  # line 23
$? = 0 << 8;
do { unlink('test_tee.txt'); $? = 0 << 8; };  # line 26
print "=== Output and Formatting Commands Complete ===\n";
$? = 0 << 8;
chdir("/") or die "cd: $!\n";  # line 29
do { require File::Path; File::Path::remove_tree($d); $? = 0 << 8; };  # line 30

exit $? >> 8;
