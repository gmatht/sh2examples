#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;
sub __sh_glob {
    my @__g = glob($_[0]);
    return @__g ? @__g : ($_[0]);
}

my $d;
my $matched;

# __SH_RUN_EXT_BEGIN (UU-FFI.md external-command seam)
# uu-ffi .so present: in-process coreutils via uu_run (capture = temp-file
# fd-1 redirect around the in-process call — no pipe, so no concurrent-
# drain deadlock, and no C-side wrapper .so needed). Absent: guarded
# fork/exec fallback (one-time warning). The bash -c below is
# feature-guarded — check_qx strips this block before scanning.
my $__SH_UU = 0;
my $__SH_UU_FFI;
our $__sh_uu_rc = 0;
my %__SH_UU_OK = map { $_ => 1 } qw(cat wc ls sort sed awk grep date sha256sum uname basename dirname cp);
BEGIN {
    my @cand;
    push @cand, $ENV{SH2_UU_LIB} if defined $ENV{SH2_UU_LIB};
    for my $up ('', '../', '../../', '../../../', '../../../../', '../../../../../') {
        push @cand, "${up}runtime/lib/libcoreutils_ffi.so";
    }
    push @cand, 'libcoreutils_ffi.so';
    for my $lib (@cand) {
        next unless defined $lib && -e $lib;
        my $ok = eval {
            require FFI::Platypus;
            my $ffi = FFI::Platypus->new(lib => [$lib]);
            $ffi->attach([uu_run => '__sh_uu_run_ffi'] => ['string', 'int', 'string[]'] => 'int');
            $__SH_UU_FFI = $ffi;
            $__SH_UU = 1;
            1;
        };
        last if $ok;
    }
    unless ($__SH_UU) {
        warn "sh2perl: uu-ffi .so (libcoreutils_ffi) not found — external commands fall back to fork/exec (slower)\n";
    }
}
my $__SH_UU_SEQ = 0;
sub __sh_uu_run {
    my @argv = @_;
    if ($__SH_UU && ($__SH_UU_OK{ $argv[0] // '' } // 0)) {
        $__sh_uu_rc = __sh_uu_run_ffi($argv[0], scalar(@argv), \@argv);
        return;
    }
    system(@argv);
    $__sh_uu_rc = $? >> 8;
}
# __SH_RUN_EXT_END

do { my $__g = q{line1
line2
TARGET
line4
line5}; my @__pl = split(/\n/, $__g); my $__m = 0; my $__r = qr/TARGET/; my @__sel; for my $__i (0..$#__pl) { my $__l = $__pl[$__i]; if ($__l =~ $__r) { $__m = 1; push @__sel, $__i; } } my %__show; for my $__i (@__sel) { for my $__j ($__i - 0..$__i + 2) { $__show{$__j} = 1 if $__j >= 0 && $__j <= $#__pl; } } for my $__i (sort { $a <=> $b } keys %__show) { print $__pl[$__i], "\n"; } $? = ($__m ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; };
do { my $__g = q{line1
line2
TARGET
line4
line5}; my @__pl = split(/\n/, $__g); my $__m = 0; my $__r = qr/TARGET/; my @__sel; for my $__i (0..$#__pl) { my $__l = $__pl[$__i]; if ($__l =~ $__r) { $__m = 1; push @__sel, $__i; } } my %__show; for my $__i (@__sel) { for my $__j ($__i - 2..$__i + 0) { $__show{$__j} = 1 if $__j >= 0 && $__j <= $#__pl; } } for my $__i (sort { $a <=> $b } keys %__show) { print $__pl[$__i], "\n"; } $? = ($__m ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; };
do { my $__g = q{line1
line2
TARGET
line4
line5}; my @__pl = split(/\n/, $__g); my $__m = 0; my $__r = qr/TARGET/; my @__sel; for my $__i (0..$#__pl) { my $__l = $__pl[$__i]; if ($__l =~ $__r) { $__m = 1; push @__sel, $__i; } } my %__show; for my $__i (@__sel) { for my $__j ($__i - 1..$__i + 1) { $__show{$__j} = 1 if $__j >= 0 && $__j <= $#__pl; } } for my $__i (sort { $a <=> $b } keys %__show) { print $__pl[$__i], "\n"; } $? = ($__m ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; };
$d = do { my $__sh_cap; $__sh_cap = do { require File::Temp; File::Temp::tempdir('sh2_ps_XXXXXX', TMPDIR => 1, CLEANUP => 0) }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 12
chdir($d) or do { exit "1"; };  # line 13
open my $__fh, '>', "temp_file1.txt" or die "Cannot write file: $!\n";
print {$__fh} "pattern in file1\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
open my $__fh, '>', "temp_file2.txt" or die "Cannot write file: $!\n";
print {$__fh} "no pattern in file2\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
open my $__fh, '>', "temp_file3.txt" or die "Cannot write file: $!\n";
print {$__fh} "pattern in file3\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "Recursive search results:\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
__sh_uu_run('grep', '-r', 'pattern', '.', __sh_glob('--include=*.txt'));
print "Result 2...\n";  # line 21
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my @__pl = split(/\n/, do { my $__capf = "/tmp/sh2_cap_1_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { __sh_uu_run('grep', '-l', 'pattern', __sh_glob('*.txt'));$? = ($__sh_uu_rc) << 8; 
 ; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }); pop @__pl if @__pl && $__pl[$#__pl] eq ''; @__pl = sort { our ($a, $b); $a cmp $b } @__pl; say join("\n", @__pl);  };
print "Result 3...\n";  # line 25
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
__sh_uu_run('grep', '-L', 'pattern', __sh_glob('*.txt'));
chdir("/") or die "cd: $!\n";  # line 29
do { require File::Path; File::Path::remove_tree($d); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };  # line 30
$matched = do { my $__sh_cap; my $__in = 'test' . "\n"; my $__n = 0; for my $__l (split(/\n/, $__in)) { $__n++ if $__l =~ qr/.*/; } $__sh_cap = $__n; $? = ($__n > 0 ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
say "  grep_exit: $CHILD_ERROR";  # line 33
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "  match_count: $matched";  # line 34
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
