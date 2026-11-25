#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $formatted_date;
my $yes_result;

print "=== System Utilities ===\n";
$? = 0 << 8;
$formatted_date = do { my $__sh_cap; $__sh_cap = (require POSIX, POSIX::strftime('%Y-%m-%d', localtime)); $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 11
say "Formatted date: $formatted_date";  # line 13
$? = 0 << 8;
print "Sleeping for " . ("1") . " seconds...\n";
$? = 0 << 8;
sleep("1");$? = 0 << 8;
$yes_result = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); print(('Hello' . "\n") x 3); $? = 0; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
print "Yes command result:\n";
$? = 0 << 8;
say $yes_result;  # line 35
$? = 0 << 8;
print "=== System Utilities Complete ===\n";
$? = 0 << 8;

exit $? >> 8;
