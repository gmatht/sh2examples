#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
my $__argc = @__sh_args;
my $file;
my $size;

sub get_file_size {  # line 3
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    my $file = $__sh_args[0];
        my $size = join(' ', split(/\s+/, do { my $__sh_cap; my $__f = $file; if (open(my $__h, '<', $__f)) { my $__d = do { local $/; <$__h> }; close($__h); $__d //= q{}; $__sh_cap = length($__d); $? = 0 << 8; } else { $__sh_cap = q{}; $? = 1 << 8; } $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap }));
    $? = 0 << 8;
    print "File " . ($file) . " has " . ($size) . " bytes\n";
    $? = 0 << 8;
}
get_file_size("test_simple_function.sh");  # line 9

exit $? >> 8;
