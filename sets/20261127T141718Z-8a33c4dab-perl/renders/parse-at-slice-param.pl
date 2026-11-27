#!/usr/bin/env perl
use strict;

use feature 'say';

sub __sh_pos {
    # bash's `${@:offset}` / `${*:offset}` — a slice of the POSITIONAL
    # PARAMETERS, which is a LIST. The renderer lowers it to
    # `join(' ', @ARGV[N..$#ARGV])`, and that join is WRONG for an empty
    # slice: `@ARGV[N..$#ARGV]` on an empty `@ARGV` is the range `N..-1`,
    # which yields NO elements, so the inner join returns '' — but the
    # OUTER join then sees that '' as ONE empty field and emits a single
    # space. bash emits nothing at all (parse-at-slice-param.sh: `' $'` vs
    # `  $'`).
    #
    # Returning the LIST, not a joined string, is what fixes it: an empty
    # list contributes zero fields to the enclosing join, and a non-empty
    # one contributes exactly its elements. The offset is 1-based, as bash
    # counts it; a non-positive or past-the-end offset is empty.
    # This sub is injected right after the `use` block, BEFORE the
    # file-scope `our @__sh_args; our $__sh_args_n;` declarations, so a bare
    # read here is a `use strict` COMPILE error and the whole program
    # aborts. Re-declaring the package globals inside the sub is always
    # legal and independent of preamble ordering (same package, so it still
    # sees the caller's values).
    our (@__sh_args, $__sh_args_n);
    my $__off = shift();
    my $__len = shift();
    return () if $__off < 1;
    return () if $__off > $__sh_args_n;
    # `@__sh_args` / `$__sh_args_n`, NOT `@ARGV` / `$#ARGV`: this helper is
    # injected AFTER `post_pass`, which is what rebinds the file-scope
    # positional reads to `@__sh_args` (a `set --` writes there, and `do
    # FILE` runs with `@ARGV` already emptied). Reading `@ARGV` here saw a
    # different (empty) list than the statements around it, so
    # parse-at-slice-param printed no space where bash printed one.
    my $__end = defined($__len) ? $__off + $__len - 1 : $__sh_args_n - 1;
    $__end = $__sh_args_n - 1 if $__end > $__sh_args_n - 1;
    return () if $__end < $__off - 1;
    return @__sh_args[$__off - 1 .. $__end];
}

our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);

my $tempfile = '';

@__sh_args = ($__sh_args[0], $tempfile, (do { my $__e = $__sh_args_n; $__e < 2 ? () : @__sh_args[2..$__e] }));  # line 3
$__sh_args_n = scalar(@__sh_args);
say join(' ', __sh_pos(1));  # line 4
$? = 0 << 8;

exit $? >> 8;
