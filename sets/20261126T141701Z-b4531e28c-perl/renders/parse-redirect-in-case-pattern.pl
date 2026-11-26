#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
my $__argc = @__sh_args;
my $__sh2_casepat_0;
my $subcommands = '';
my %words;
my $counter = '';
my $words = '';

sub _docker_to_extglob {
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    return $__sh_args[0];
}
$__sh2_casepat_0 = _docker_to_extglob($subcommands);
if (($words{$counter}) =~ /^(?:\Q$__sh2_casepat_0\E)$/) {
    print "match\n";
    $? = 0 << 8;
    }

exit $? >> 8;
