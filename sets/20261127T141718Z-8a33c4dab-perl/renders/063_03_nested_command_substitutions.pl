#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $output = '';

$output = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     print("Result: " . (do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     print("Nested: " . (do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     print("Deep: " . (do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     print "Level 4\n";
    $? = 0 << 8;
;; } $__cap =~ s/\n+\z//; $__cap }), "\n");
    $? = 0 << 8;
;; } $__cap =~ s/\n+\z//; $__cap }), "\n");
    $? = 0 << 8;
;; } $__cap =~ s/\n+\z//; $__cap }), "\n");
    $? = 0 << 8;
;; } $__cap =~ s/\n+\z//; $__cap };
say $output;  # line 5
$? = 0 << 8;

exit $? >> 8;
