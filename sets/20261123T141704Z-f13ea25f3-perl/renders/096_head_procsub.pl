#!/usr/bin/env perl
use strict;
use warnings;
my $main_exit_code = 0;
our $CHILD_ERROR = 0;
use feature 'say';

our $__sh2_last_bg = '';
my $__ps_tmp0;
my $__ps_bgp0;
my $__ps_status0;

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

$__ps_tmp0 = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
require POSIX;
unlink($__ps_tmp0); POSIX::mkfifo($__ps_tmp0, 0600) or die "mkfifo: $!\n";
$| = 1;
my $__sh2_bg1 = fork();
$__sh2_last_bg = $__sh2_bg1;
if (defined $__sh2_bg1 && $__sh2_bg1 == 0) {
    open(my $__sh2_rfh1, '>', $__ps_tmp0) or die "Cannot write to __ps_tmp0: $!\n";
    my $__sh2_rsel1 = select($__sh2_rfh1);
    $| = 1;
    while (1) {
                print ".\n";
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        sleep("1");
    }
    select($__sh2_rsel1); close($__sh2_rfh1);
    exit $main_exit_code;
}
$__ps_bgp0 = $__sh2_last_bg;
__sh_uu_run('head', $__ps_tmp0);$? = ($__sh_uu_rc) << 8; $CHILD_ERROR = $? >> 8;
$__ps_status0 = $CHILD_ERROR;
do { my $__sh_killed = kill('TERM', $__ps_bgp0); };
waitpid($__ps_bgp0, 0);
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { unlink($__ps_tmp0); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
$CHILD_ERROR = $__ps_status0;
say "exit: $CHILD_ERROR";  # line 5
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
