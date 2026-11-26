#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

sub __sh_glob {
    my @__g = glob($_[0]);
    return @__g ? @__g : ($_[0]);
}

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

print "== Basic brace expansion ==\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "1 2 3 4 5\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "a b c\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "00 02 04\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "== Advanced brace expansion ==\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "a1 a2 a3 b1 b2 b3 c1 c2 c3\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "1 3 5 7 9\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "a d g j m p s v y\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "== Practical examples ==\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my $__ok = 1; for my $__t ('file_001.txt', 'file_002.txt', 'file_003.txt', 'file_004.txt', 'file_005.txt') { open(my $__h, '>>', $__t) or ($__ok = 0); close($__h) if $__ok; } $? = ($__ok ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; };
__sh_uu_run('ls', __sh_glob('file_*.txt'));  # line 21
__sh_uu_run('rm', __sh_glob('file_*.txt'));$? = ($__sh_uu_rc) << 8; $CHILD_ERROR = $? >> 8;  # line 22
say "brace_expand: 1 2 3 4 5";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
