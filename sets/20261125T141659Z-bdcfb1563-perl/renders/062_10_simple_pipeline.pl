#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;
my $d;

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

print "Testing simple pipeline...\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$d = do { my $__sh_cap; $__sh_cap = do { require File::Temp; File::Temp::tempdir('sh2_ps_XXXXXX', TMPDIR => 1, CLEANUP => 0) }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 5
do { require File::Path; File::Path::make_path(($d) . "/dir1", ($d) . "/dir2", ($d) . "/dir3"); my $__ok = 1; $? = ($__ok ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; };  # line 6
do { my $__ok = 1; for my $__t (($d) . "/file1", ($d) . "/file2") { open(my $__h, '>>', $__t) or ($__ok = 0); close($__h) if $__ok; } $? = ($__ok ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; };  # line 7
chdir($d) or die "cd: $!\n";  # line 8
my $__sh_pv = do { my $__capf = "/tmp/sh2_cap_3_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { do { my $__g = do { my $__capf = "/tmp/sh2_cap_2_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { __sh_uu_run('ls', '-l');$? = ($__sh_uu_rc) << 8; $CHILD_ERROR = $? >> 8;
 ; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }; my $__m = 0; for my $__l (split(/\n/, $__g)) { if ($__l =~ /^d/) { print $__l, "\n"; $__m = 1; } } $? = ($__m ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; }; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }; do { my @__pl = split(/\n/, $__sh_pv); pop @__pl if @__pl && $__pl[-1] eq ''; @__pl = @__pl[0..4] if @__pl > 5; say join("\n", @__pl) if @__pl; $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
say "exit: $CHILD_ERROR";  # line 11
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
chdir("/") or die "cd: $!\n";  # line 12
do { require File::Path; File::Path::remove_tree($d); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };  # line 13

exit $? >> 8;
