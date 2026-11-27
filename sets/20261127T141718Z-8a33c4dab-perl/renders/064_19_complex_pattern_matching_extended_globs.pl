#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

sub __sh_glob {
    my @__g = glob($_[0]);
    return @__g ? @__g : ($_[0]);
}

my $d;
my $file;

$d = do { my $__sh_cap; $__sh_cap = do { require File::Temp; File::Temp::tempdir('sh2_ps_XXXXXX', TMPDIR => 1, CLEANUP => 0) }; $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 6
chdir($d) or do { warn "cd: " . $d . ": $!\n"; exit "1"; };  # line 7
do { my $__ok = 1; for my $__t ('a.txt', 'b.log', 'c.dat', 'x.yz') { if (open(my $__h, '>>', $__t)) { close($__h); } else { $__ok = 0; print STDERR "touch: cannot touch '$__t': $!\n"; } } $? = ($__ok ? 0 : 1) << 8; };  # line 8
for my $__file ((__sh_glob('*.txt'), __sh_glob('*.log'), __sh_glob('*.dat'))) {
    $file = $__file;
    if (($file) =~ /^(?:.*\.txt|.*\.log)$/) {
        say "Text file: $file";
        $? = 0 << 8;
            } elsif (($file) =~ /^(?:.*\.dat)$/) {
        say "Data file: $file";
        $? = 0 << 8;
            } else {
        say "Other file: $file";
        $? = 0 << 8;
            }
}
if (chdir("/")) {  # line 16
    
} else {
    warn "cd: " . "/" . ": $!\n";
    
}
do { require File::Path; File::Path::remove_tree($d); $? = 0 << 8; };  # line 17

exit $? >> 8;
