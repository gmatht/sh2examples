#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;
my $X_SET;
my $arg;

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

$X_SET = "0";  # line 4
$ENV{X_SET} = ${X_SET} if exists $ENV{X_SET};
my $__sh_loop_rc0 = 0;  # line 5
for my $__arg ((@ARGV)) {
    $arg = $__arg;
    if (pipeline((sub {     say $arg;
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}, sub {     __sh_uu_run('grep', '-q', '-e', '^-[a-zA-Z0-9]*x');$? = ($__sh_uu_rc) << 8; $CHILD_ERROR = $? >> 8;
}))) {
        $X_SET = "1";
        $ENV{X_SET} = ${X_SET} if exists $ENV{X_SET};
    }
    $__sh_loop_rc0 = $? >> 8;
}
$? = ($__sh_loop_rc0) << 8; $CHILD_ERROR = $? >> 8;
say $X_SET;  # line 10
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
