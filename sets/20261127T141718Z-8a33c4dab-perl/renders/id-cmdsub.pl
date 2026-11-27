#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
use feature 'say';

our $CHILD_ERROR = 0;
my $__argc = @__sh_args;
my $label;
my $tmp_stdout;
my $tmp_stderr;
my $ec;
my $so;
my $se;

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
sub __sh_uu_capture {
    my @argv = @_;
    if ($__SH_UU && ($__SH_UU_OK{ $argv[0] // '' } // 0)) {
        # Capture = fd-1 redirect to a temp file around the IN-PROCESS
        # uu_run call (the fd redirect is what the C wrapper's drain
        # thread exists for; a temp file cannot fill, so no deadlock).
        my $tmpf = "/tmp/sh2_uu_${$}_" . ($__SH_UU_SEQ++) . ".cap";
        open(my $saved, '>&', \*STDOUT) or do { $__sh_uu_rc = 127; return ''; };
        open(STDOUT, '>', $tmpf) or do { $__sh_uu_rc = 127; return ''; };
        my $rc = eval { __sh_uu_run_ffi($argv[0], scalar(@argv), \@argv) };
        open(STDOUT, '>&', $saved) or do { $__sh_uu_rc = 127; return ''; };
        close($saved);
        $__sh_uu_rc = defined $rc ? $rc : 127;
        my $out = '';
        if (open(my $rf, '<', $tmpf)) { $out = do { local $/; <$rf> } // ''; close $rf; }
        unlink $tmpf;
        return $out;
    }
    my $out = do {
        open(my $fh, '-|', @argv) or do { $__sh_uu_rc = 127; return ''; };
        my $r = do { local $/; <$fh> };
        close $fh;
        $r;
    };
    $__sh_uu_rc = $? >> 8;
    return $out // '';
}
# __SH_RUN_EXT_END

$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 13
sub capture {
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    my $label = $__sh_args[0];
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    shift @__sh_args;
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    my $tmp_stdout;
    my $tmp_stderr;
    $tmp_stdout = do { my $_r = __sh_uu_capture('mktemp', '/tmp/id_demo_stdout_XXXXXX'); $CHILD_ERROR = $__sh_uu_rc; $_r =~ s/\n+\z//; $_r; };
    $tmp_stderr = do { my $_r = __sh_uu_capture('mktemp', '/tmp/id_demo_stderr_XXXXXX'); $CHILD_ERROR = $__sh_uu_rc; $_r =~ s/\n+\z//; $_r; };
    { open(my $__sh2_dupfh0, '>&', \*STDOUT) or die; open(STDOUT, '>', $tmp_stdout) or die "redirect: $!\n"; open(my $__sh2_dupfh1, '>&', \*STDERR) or die; open(STDERR, '>', $tmp_stderr) or die "redirect: $!\n"; system(@__sh_args);$CHILD_ERROR = $? >> 8; open(STDOUT, '>&', $__sh2_dupfh0) or die; close($__sh2_dupfh0); open(STDERR, '>&', $__sh2_dupfh1) or die; close($__sh2_dupfh1); }
    my $ec = $CHILD_ERROR;
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    my $so;
    $so = do { my $__sh_cap; if (open(my $__h, '<', $tmp_stdout)) { local $/; my $__d = <$__h>; close($__h); $__sh_cap = defined($__d) ? $__d : q{}; $? = 0 << 8; $CHILD_ERROR = $? >> 8; } else { $__sh_cap = q{}; $? = 1 << 8; $CHILD_ERROR = $? >> 8; } $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
    my $se;
    $se = do { my $__sh_cap; if (open(my $__h, '<', $tmp_stderr)) { local $/; my $__d = <$__h>; close($__h); $__sh_cap = defined($__d) ? $__d : q{}; $? = 0 << 8; $CHILD_ERROR = $? >> 8; } else { $__sh_cap = q{}; $? = 1 << 8; $CHILD_ERROR = $? >> 8; } $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
    do { unlink($tmp_stdout, $tmp_stderr); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
    say "--- [$label] ---";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    print("  cmd     : " . (join(' ', @__sh_args)), "\n");
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    say "  exitcode: $ec";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    if ($so ne "") {
        say "  stdout  : $so";
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    }
    if ($se ne "") {
        say "  stderr  : $se";
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    }
    print "\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    return $ec;
}
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 1: 'id' (default — current process)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("01-default", "id");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 2: id -u  (effective user ID, numeric)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("02-u", "id", "-u");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 3: id -g  (effective group ID, numeric)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("03-g", "id", "-g");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 4: id -G  (all group IDs, numeric)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("04-G", "id", "-G");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 5: id -un  (effective user name)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("05-un", "id", "-un");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 6: id -gn  (effective group name)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("06-gn", "id", "-gn");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 7: id -Gn  (all group names)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("07-Gn", "id", "-Gn");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 8: id -ru  (real user ID, numeric)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("08-ru", "id", "-ru");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 9: id -rg  (real group ID, numeric)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("09-rg", "id", "-rg");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 10: id -rG  (real group IDs, numeric)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("10-rG", "id", "-rG");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 11: id -run  (real user name)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("11-run", "id", "-run");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 12: id -rgn  (real group name)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("12-rgn", "id", "-rgn");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 13: id -rGn  (real group names)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("13-rGn", "id", "-rGn");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 14: id -u -n  (separate options, equivalent to -un)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("14-u-n", "id", "-u", "-n");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 15: id -r -u  (separate options, equivalent to -ru)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("15-r-u", "id", "-r", "-u");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 16: id -G -z  (group IDs delimited by NUL)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("16-Gz", "bash", "-c", 'id -G -z | cat -v');
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 17: id -Gn -z  (group names delimited by NUL)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("17-Gnz", "bash", "-c", 'id -Gn -z | cat -v');
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 18: id -un -z  (user name with NUL terminator)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("18-unz", "bash", "-c", 'id -un -z | cat -v');
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 19: id -Z  (security context — may not be available)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("19-Z", "id", "-Z");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 20: id root  (default output for user 'root')\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("20-root", "id", "root");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 21: id -u root  (numeric UID of root)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("21-u-root", "id", "-u", "root");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 22: id -g root  (numeric GID of root)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("22-g-root", "id", "-g", "root");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 23: id -G root  (all group IDs of root, numeric)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("23-G-root", "id", "-G", "root");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 24: id -un root  (user name of root)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("24-un-root", "id", "-un", "root");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 25: id -ru root  (real user ID of root, numeric)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("25-ru-root", "id", "-ru", "root");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 26: id -a  (compatibility flag — same as default)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("26-a", "id", "-a");
print "\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "All id demo sections completed.\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
exit "0";

exit $? >> 8;
