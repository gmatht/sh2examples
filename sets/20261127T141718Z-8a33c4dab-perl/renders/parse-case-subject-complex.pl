#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);

$? = 0 << 8;  # line 3
if ((($__sh_args[0]) . "-" . (do { my $__sh_cap; $__sh_cap = (require POSIX, (POSIX::uname())[0]); $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap })) =~ /^(?:regex-Linux)$/) {
    print "Linux\n";
    $? = 0 << 8;
    $? = 0 << 8;
} elsif ((($__sh_args[0]) . "-" . (do { my $__sh_cap; $__sh_cap = (require POSIX, (POSIX::uname())[0]); $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap })) =~ /^(?:regex-.*)$/) {
    print("Other: " . (do { my $__sh_cap; $__sh_cap = (require POSIX, (POSIX::uname())[0]); $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap }), "\n");
    $? = 0 << 8;
}

exit $? >> 8;
