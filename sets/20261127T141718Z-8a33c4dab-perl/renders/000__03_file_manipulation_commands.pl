#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $d;
my $cp_result;
my $mv_result;
my $rm_result;
my $mkdir_result;
my $touch_result;

print "=== File Manipulation Commands ===\n";
$? = 0 << 8;
print "=== cp command ===\n";
$? = 0 << 8;
$d = do { my $__sh_cap; $__sh_cap = do { require File::Temp; File::Temp::tempdir('sh2_ps_XXXXXX', TMPDIR => 1, CLEANUP => 0) }; $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 11
chdir($d) or do { warn "cd: " . $d . ": $!\n"; exit "1"; };  # line 12
print "\n";  # line 13
$? = 0 << 8;
open my $__fh, '>', "test_file.txt" or die "Cannot write file: $!\n";
print {$__fh} "test content\n";
close $__fh;
$? = 0 << 8;
$cp_result = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     do { require File::Copy; my $__ok = File::Copy::copy('test_file.txt', 'test_file_copy.txt'); $? = ($__ok ? 0 : 1) << 8; };
    if ($? == 0) {
        print "Copy successful\n";
        $? = 0 << 8;
        $? = 0 << 8;
    }
;; } $__cap =~ s/\n+\z//; $__cap };
say "Copy result: $cp_result";  # line 16
$? = 0 << 8;
my $__lsrc0 = 0; do { my $__lsbad0 = 0; my @__lp0 = ('test_file.txt', 'test_file_copy.txt', 'test_file_moved.txt'); my $__multi = 1; for my $__p0 (@__lp0) { if (!-e $__p0) { $__lsbad0 = 1; next; } if (-d $__p0) { if ($__multi) { print STDOUT "$__p0:\n"; } opendir(my $__dh0, $__p0) or next; my @__d0 = sort grep { $_ ne '.' && $_ ne '..' } readdir($__dh0); closedir $__dh0; if (@__d0) { print STDOUT join("\n", @__d0), "\n"; } } else { print STDOUT $__p0, "\n"; } } $__lsrc0 = $__lsbad0 ? 2 : 0; }; if ($__lsrc0) { print STDOUT 'No test files found', "\n"; } $? = ($__lsrc0) << 8;print "\n";  # line 19
$? = 0 << 8;
print "=== mv command ===\n";
$? = 0 << 8;
$mv_result = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     do { my $__ok = rename('test_file_copy.txt', 'test_file_moved.txt'); $? = ($__ok ? 0 : 1) << 8; };
    if ($? == 0) {
        print "Move successful\n";
        $? = 0 << 8;
        $? = 0 << 8;
    }
;; } $__cap =~ s/\n+\z//; $__cap };
say "Move result: $mv_result";  # line 24
$? = 0 << 8;
my $__lsrc1 = 0; do { my $__lsbad1 = 0; my @__lp1 = ('test_file.txt', 'test_file_copy.txt', 'test_file_moved.txt'); my $__multi = 1; for my $__p1 (@__lp1) { if (!-e $__p1) { $__lsbad1 = 1; next; } if (-d $__p1) { if ($__multi) { print STDOUT "$__p1:\n"; } opendir(my $__dh1, $__p1) or next; my @__d1 = sort grep { $_ ne '.' && $_ ne '..' } readdir($__dh1); closedir $__dh1; if (@__d1) { print STDOUT join("\n", @__d1), "\n"; } } else { print STDOUT $__p1, "\n"; } } $__lsrc1 = $__lsbad1 ? 2 : 0; }; if ($__lsrc1) { print STDOUT 'No test files found', "\n"; } $? = ($__lsrc1) << 8;print "\n";  # line 27
$? = 0 << 8;
print "=== rm command ===\n";
$? = 0 << 8;
$rm_result = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     do { my $__ok = 1; for my $__p ('test_file.txt', 'test_file_moved.txt') { unless (unlink($__p)) { $__ok = 0; print STDERR "rm: cannot remove '$__p': $!\n"; } } $? = ($__ok ? 0 : 1) << 8; };
    if ($? == 0) {
        print "Remove successful\n";
        $? = 0 << 8;
        $? = 0 << 8;
    }
;; } $__cap =~ s/\n+\z//; $__cap };
say "Remove result: $rm_result";  # line 32
$? = 0 << 8;
my $__lsrc2 = 0; do { my $__lsbad2 = 0; my @__lp2 = ('test_file.txt', 'test_file_copy.txt', 'test_file_moved.txt'); my $__multi = 1; for my $__p2 (@__lp2) { if (!-e $__p2) { $__lsbad2 = 1; next; } if (-d $__p2) { if ($__multi) { print STDOUT "$__p2:\n"; } opendir(my $__dh2, $__p2) or next; my @__d2 = sort grep { $_ ne '.' && $_ ne '..' } readdir($__dh2); closedir $__dh2; if (@__d2) { print STDOUT join("\n", @__d2), "\n"; } } else { print STDOUT $__p2, "\n"; } } $__lsrc2 = $__lsbad2 ? 2 : 0; }; if ($__lsrc2) { print STDOUT 'No test files found', "\n"; } $? = ($__lsrc2) << 8;print "\n";  # line 35
$? = 0 << 8;
print "=== mkdir command ===\n";
$? = 0 << 8;
$mkdir_result = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     do { my $__ok = 1; for my $__d ('test_dir') { unless (mkdir($__d)) { $__ok = 0; print STDERR "mkdir: $!\n"; } } $? = ($__ok ? 0 : 1) << 8; };
    if ($? == 0) {
        print "Directory created\n";
        $? = 0 << 8;
        $? = 0 << 8;
    }
;; } $__cap =~ s/\n+\z//; $__cap };
say "Mkdir result: $mkdir_result";  # line 40
$? = 0 << 8;
do { my $__ok = 1; for my $__t ('test_dir/file') { if (open(my $__h, '>>', $__t)) { close($__h); } else { $__ok = 0; print STDERR "touch: cannot touch '$__t': $!\n"; } } $? = ($__ok ? 0 : 1) << 8; };  # line 41
my $__lsrc3 = 0; do { my $__lsbad3 = 0; my @__lp3 = ('test_dir'); my $__multi = 0; for my $__p3 (@__lp3) { if (!-e $__p3) { $__lsbad3 = 1; next; } if (-d $__p3) { if ($__multi) { print STDOUT "$__p3:\n"; } opendir(my $__dh3, $__p3) or next; my @__d3 = sort grep { $_ ne '.' && $_ ne '..' } readdir($__dh3); closedir $__dh3; if (@__d3) { print STDOUT join("\n", @__d3), "\n"; } } else { print STDOUT $__p3, "\n"; } } $__lsrc3 = $__lsbad3 ? 2 : 0; }; if ($__lsrc3) { print STDOUT 'Directory not found', "\n"; } $? = ($__lsrc3) << 8;do { my $__ok = 1; for my $__p ('test_dir/file') { unless (unlink($__p)) { $__ok = 0; print STDERR "rm: cannot remove '$__p': $!\n"; } } $? = ($__ok ? 0 : 1) << 8; };  # line 43
do { my $__ok = 1; for my $__d ('test_dir') { unless (rmdir($__d)) { $__ok = 0; print STDERR "rmdir: failed to remove '$__d': $!\n"; } } $? = ($__ok ? 0 : 1) << 8; };  # line 44
print "\n";  # line 46
$? = 0 << 8;
print "=== touch command ===\n";
$? = 0 << 8;
$touch_result = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     do { my $__ok = 1; for my $__t ('test_file.txt') { if (open(my $__h, '>>', $__t)) { close($__h); } else { $__ok = 0; print STDERR "touch: cannot touch '$__t': $!\n"; } } $? = ($__ok ? 0 : 1) << 8; };
    if ($? == 0) {
        print "File touched\n";
        $? = 0 << 8;
        $? = 0 << 8;
    }
;; } $__cap =~ s/\n+\z//; $__cap };
say "Touch result: $touch_result";  # line 51
$? = 0 << 8;
print "\n";  # line 53
$? = 0 << 8;
do { unlink('test_file.txt', 'test_file_copy.txt', 'test_file_moved.txt'); $? = 0 << 8; };  # line 55
do { my $__rm; $__rm = sub { my ($__d) = @_; if (-d $__d) { opendir(my $__h, $__d) or return; for my $__e (readdir($__h)) { next if $__e eq '.' || $__e eq '..'; my $__p = "$__d/$__e"; if (-d $__p) { $__rm->($__p); } else { unlink $__p; } } closedir $__h; rmdir $__d; } else { unlink $__d; } }; if (-e 'test_dir') { $__rm->('test_dir'); } }; $? = 0 << 8;if (chdir("/")) {  # line 57
    
} else {
    warn "cd: " . "/" . ": $!\n";
    
}
do { require File::Path; File::Path::remove_tree($d); $? = 0 << 8; };  # line 58

exit $? >> 8;
