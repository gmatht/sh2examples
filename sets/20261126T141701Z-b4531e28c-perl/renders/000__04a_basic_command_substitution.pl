#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $current_date;
my $current_dir;

print "=== Basic Command Substitution ===\n";
$? = 0 << 8;
print("Current date: " . (do { my $__sh_cap; $__sh_cap = (require POSIX, POSIX::strftime('%Y', localtime)); $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap }), "\n");  # line 9
print("Current directory: " . (do { my $__sh_cap; $__sh_cap = (do { require File::Basename; File::Basename::basename(do { my $__sh_cap; $__sh_cap = (require Cwd, Cwd::getcwd()); $? = 0 << 8; $__sh_cap }) }); $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap }), "\n");  # line 11
$? = 0 << 8;
$current_date = do { my $__sh_cap; $__sh_cap = (require POSIX, POSIX::strftime('%Y%m', localtime)); $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 14
$current_dir = do { my $__sh_cap; $__sh_cap = (do { require File::Basename; File::Basename::basename(do { my $__sh_cap; $__sh_cap = (require Cwd, Cwd::getcwd()); $? = 0 << 8; $__sh_cap }) }); $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 16
say "Stored date: $current_date";  # line 18
$? = 0 << 8;
say "Stored directory: $current_dir";  # line 20
$? = 0 << 8;
print "=== Basic Command Substitution Complete ===\n";
$? = 0 << 8;

exit $? >> 8;
