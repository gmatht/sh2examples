#!/usr/bin/env perl
use strict;


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

sub __sh_fn_status {
    # A shell function called in CONDITION position is tested by its exit
    # status (`if f x; then …`), not by its return string: `return 0` means
    # success. Perl would read `return "0"` as the false string "0" and
    # invert the branch, so test the status explicitly. Mirrors the bash
    # convention the rest of the program already uses: 0 => branch taken.
    #
    # Argv: ($name, @args) — the renderer keeps the function name as the first
    # word of the call. A string is not a code ref under `use strict 'refs'`,
    # so resolve it through the symbol table.
    my $name = shift();
    no strict 'refs';
    my $rv = &{$name}(@_);
    # A function that `return`s a bare integer is signalling status; one that
    # returns something else (or nothing) is signalling via $?.
    my $st = defined($rv) && $rv =~ /^\s*-?[0-9]+\s*$/
        ? $rv + 0
        : ($? >> 8);
    $st = 1 if !defined $st;
    # This sub is injected right after the `use` block, BEFORE the shared
    # renderer's own preamble (`our $CHILD_ERROR = 0;`), so a bare
    # $CHILD_ERROR here is a `use strict` COMPILE error ("Global symbol
    # $CHILD_ERROR requires explicit package name") and the whole program
    # aborts — 051_primes printed NOTHING. Re-declaring the package global
    # inside the sub is always legal, makes the helper independent of
    # preamble ordering, and (same package) still writes the caller's
    # variable.
    our $CHILD_ERROR;
    $CHILD_ERROR = $st;
    return $st == 0 ? 1 : 0;
}

our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
use feature 'say';

my $__argc = @__sh_args;
my $y;
my @z;
my $n;
my $sqrt_n;
my $i;
my @primes;
my $count;
my $candidate;

$y = $y . "2";  # line 7
push @z, ("a", "b");  # line 8
push @z, (__sh_slice(\@primes, 0, 1));  # line 9
print "=== Prime Number Generator (first 1000 primes) ===\n";
$? = 0 << 8;
sub is_prime {
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    my $n = $__sh_args[0];
        if (int($n) < int(2)) {
        $? = 1 << 8;
        return 1;
    }
    if (int($n) == int(2)) {
        $? = 0 << 8;
        return 0;
    }
    if ((do { my $__a = (do { my $__d = 2; eval { int(($n)) % ($__d) } }); defined($__a) ? int($__a) : undef }) == int(0)) {
        $? = 1 << 8;
        return 1;
    }
    my $sqrt_n = join(' ', split(/\s+/, do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); print int(sqrt(($n))), "\n"; $? = 0; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap }));
        my $i = "3";
    my $__sh_loop_rc0 = 0;
    while (int($i) <=int($sqrt_n)) {
        if ((0+(do { my $__d = $i; eval { int(($n)) % ($__d) } })) == (0+'0')) {
            $? = 1 << 8;
            return 1;
        }
        $i = int(($i + 2));
        $__sh_loop_rc0 = $? >> 8;
    }
$? = ($__sh_loop_rc0) << 8;
    $? = 0 << 8;
    return 0;
}
print "Finding first 100 prime numbers...\n";
$? = 0 << 8;
print "This may take a while...\n";
$? = 0 << 8;
@primes = ("2");  # line 45
$count = "1";  # line 46
$candidate = "3";  # line 47
while (int($count) < int(100)) {
    if (__sh_fn_status("is_prime", ($candidate))) {
        push @primes, (split(/\s+/, $candidate));
        $count = int(($count + 1));
        if ((do { my $__a = (do { my $__d = 10; eval { int(($count)) % ($__d) } }); defined($__a) ? int($__a) : undef }) == int(0)) {
            print "Found " . ($count) . " primes so far...\n";
            $? = 0 << 8;
        }
    }
    $candidate = int(($candidate + 2));
}
print "\n";
$? = 0 << 8;
print "First 1000 prime numbers found!\n";
$? = 0 << 8;
print("Count: " . (join(' ', scalar(@primes))), "\n");
print("First 10: " . (join(' ', __sh_slice(\@primes, 0, 10))), "\n");  # line 65
print("Last 10: " . (join(' ', __sh_slice(\@primes, -10, undef))), "\n");  # line 66
$? = 0 << 8;
print "Prime number generation complete!\n";
$? = 0 << 8;

exit $? >> 8;
