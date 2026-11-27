#!/usr/bin/env perl
use strict;
use warnings;

use feature 'say';

sub __sh_slice {
    # bash's `${arr[@]:offset:length}` semantics, which a Perl range bound
    # does not reproduce:
    #
    #   * a NEGATIVE offset counts back from the last element, and the slice
    #     ends at the last element (not at `$#arr`);
    #   * an offset before the first element yields the EMPTY list;
    #   * a length past the end stops at the last element — a Perl range would
    #     instead pad with undef, which `join` renders as empty fields.
    #
    # So both bounds are clamped against the real element count here.
    my $__av = shift();   # \@name
    my $__off = shift();
    my $__len = shift();
    my $__n = scalar(@$__av);
    my $__start = $__off < 0 ? $__n + $__off : $__off;
    return () if $__start < 0;
    return () if $__start > $__n;
    return @$__av[$__start .. $__n - 1] if !defined($__len);
    return () if $__len <= 0;
    my $__end = $__start + $__len - 1;
    $__end = $__n - 1 if $__end > $__n - 1;
    return @$__av[$__start .. $__end];
}

my $var = '';
my $default = '';
my $fallback = '';
my %array;
my @default;
my $index = '';
my %prefix;
my $array = '';

say ((defined($var) && length($var)) ? $var : ((defined($default) && length($default)) ? $default : ((defined($fallback) && length($fallback)) ? $fallback : "computed")));
say ((defined($array{$index}) && length($array{$index})) ? $array{$index} : join(' ', __sh_slice(\@default, 0, 2)));  # line 5
$? = 1 << 8;  # line 6

exit $? >> 8;
