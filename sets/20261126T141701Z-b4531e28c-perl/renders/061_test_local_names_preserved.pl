#!/usr/bin/env perl
use strict;
use feature 'say';

our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
my $__argc = @__sh_args;
my $first_number;
my $second_number;
my $operation;
my $input_string;
my $search_pattern;
my $replacement;
my $array_name;
my $index;
my $new_value;

sub test_math {
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    my $first_number = $__sh_args[0];
        my $second_number = $__sh_args[1];
        my $operation = $__sh_args[2];
    $? = 0 << 8;
    if (($operation) =~ /^(?:add)$/) {
        return int(($first_number + $second_number));
    } elsif (($operation) =~ /^(?:subtract)$/) {
        return int(($first_number - $second_number));
    } elsif (($operation) =~ /^(?:multiply)$/) {
        return int(($first_number * $second_number));
    } else {
        return "Unknown operation: " . ($operation);
    }
}
sub test_strings {
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    my $input_string = $__sh_args[0];
        my $search_pattern = $__sh_args[1];
        my $replacement = $__sh_args[2];
    $? = 0 << 8;
    if (($search_pattern) =~ /^(?:start)$/) {
        return "Replacing start of: " . ($input_string) . " with: " . ($replacement);
    } elsif (($search_pattern) =~ /^(?:end)$/) {
        return "Replacing end of: " . ($input_string) . " with: " . ($replacement);
    } elsif (($search_pattern) =~ /^(?:middle)$/) {
        return "Replacing middle of: " . ($input_string) . " with: " . ($replacement);
    } else {
        return "Unknown pattern: " . ($search_pattern) . " for string: " . ($input_string);
    }
}
sub test_arrays {
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    my $array_name = $__sh_args[0];
        my $index = $__sh_args[1];
        my $new_value = $__sh_args[2];
    $? = 0 << 8;
    if (($index) =~ /^(?:first)$/) {
        return "Setting first element of " . ($array_name) . " to " . ($new_value);
    } elsif (($index) =~ /^(?:last)$/) {
        return "Setting last element of " . ($array_name) . " to " . ($new_value);
    } else {
        return "Setting element " . ($index) . " of " . ($array_name) . " to " . ($new_value);
    }
}
say test_math("10", "5", "add");
say test_math("10", "5", "multiply");
say test_strings("hello world", "start", "hi");
say test_strings("hello world", "end", "bye");
say test_arrays("my_array", "first", "new_value");
say test_arrays("my_array", "last", "final_value");
$? = 0 << 8;

exit $? >> 8;
