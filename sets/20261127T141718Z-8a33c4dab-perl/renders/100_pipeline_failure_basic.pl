#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $d;

$d = do { my $__sh_cap; $__sh_cap = do { require File::Temp; File::Temp::tempdir('sh2_ps_XXXXXX', TMPDIR => 1, CLEANUP => 0) }; $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 4
chdir($d) or do { warn "cd: " . $d . ": $!\n"; exit "1"; };  # line 5
open my $__fh, '>', "alpha" or die "Cannot write file: $!\n";  # line 6
print {$__fh} "alpha\n";
close $__fh;
$? = 0 << 8;
open my $__fh, '>', "beta" or die "Cannot write file: $!\n";  # line 7
print {$__fh} "beta\n";
close $__fh;
$? = 0 << 8;
open my $__fh, '>', "gamma" or die "Cannot write file: $!\n";  # line 8
print {$__fh} "gamma\n";
close $__fh;
$? = 0 << 8;
print "File list:\n";
$? = 0 << 8;
do { opendir(my $__h, '.') or return; my @__l = sort grep { !/^\.\.?$/ } readdir($__h); closedir $__h; splice(@__l, 3) if @__l > 3; if (@__l) { print STDOUT join("\n", @__l), "\n"; } }; $? = 0 << 8;print "---\n";
$? = 0 << 8;
print "Count:\n";
$? = 0 << 8;
do { opendir(my $__h, '.') or return; my $__n = grep { !/^\.\.?$/ } readdir($__h); closedir $__h; print STDOUT "$__n\n"; }; $? = 0 << 8;print "done\n";
$? = 0 << 8;
if (chdir("/")) {  # line 15
    
} else {
    warn "cd: " . "/" . ": $!\n";
    
}
do { require File::Path; File::Path::remove_tree($d); $? = 0 << 8; };  # line 16

exit $? >> 8;
