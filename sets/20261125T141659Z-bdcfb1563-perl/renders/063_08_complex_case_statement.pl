#!/usr/bin/env perl
use strict;
use warnings;
my $var = '';

if ((do { my $__cap = do { my $__t = $var . "\n"; $__t =~ tr/A-Z/a-z/; $__t }; $__cap =~ s/\n+\z//; $__cap }) =~ /^(?:.*[0-9].*)$/) {  # line 4
    if ((lc($var)) =~ /^(?:.*pattern.*)$/) {
        print "Double nested pattern\n";
        $? = 0 << 8;
            } else {
        print "Single nested pattern\n";
        $? = 0 << 8;
            }
} else {
    print "No numbers\n";
    $? = 0 << 8;
    }

exit $? >> 8;
