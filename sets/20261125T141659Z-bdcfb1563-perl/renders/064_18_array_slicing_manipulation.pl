#!/usr/bin/env perl
use strict;
use warnings;

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

my @numbers;
my @middle;
my @first_half;
my @last_half;

@numbers = ("1", "2", "3", "4", "5", "6", "7", "8", "9", "10");  # line 4
@middle = (join(' ', __sh_slice(\@numbers, 3, 4)));  # line 5
@first_half = (join(' ', __sh_slice(\@numbers, 0, 5)));  # line 6
@last_half = (join(' ', @numbers[5..$#numbers]));  # line 7
print("Middle: " . (join(' ', @middle)), "\n");  # line 9
print("First half: " . (join(' ', @first_half)), "\n");  # line 10
print("Last half: " . (join(' ', @last_half)), "\n");  # line 11
$? = 0 << 8;

exit $? >> 8;
