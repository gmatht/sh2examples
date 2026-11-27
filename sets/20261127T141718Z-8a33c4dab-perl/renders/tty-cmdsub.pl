#!/usr/bin/env perl
use strict;
sub __sh_glob {
    my @__g = glob($_[0]);
    return @__g ? @__g : ($_[0]);
}

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
my $TTY_DEV;
my $dev;

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

$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 16
sub capture {
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    my $label = $__sh_args[0];
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    shift @__sh_args;
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    my $tmp_stdout;
    my $tmp_stderr;
    $tmp_stdout = do { my $_r = __sh_uu_capture('mktemp', '/tmp/tty_demo_stdout_XXXXXX'); $CHILD_ERROR = $__sh_uu_rc; $_r =~ s/\n+\z//; $_r; };
    $tmp_stderr = do { my $_r = __sh_uu_capture('mktemp', '/tmp/tty_demo_stderr_XXXXXX'); $CHILD_ERROR = $__sh_uu_rc; $_r =~ s/\n+\z//; $_r; };
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
    } else {
        print "  stdout  : (empty)\n";
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
$TTY_DEV = "";
$ENV{TTY_DEV} = ${TTY_DEV} if exists $ENV{TTY_DEV};
my $__sh_loop_rc0 = 0;  # line 54
for my $__dev (("/dev/pts/2", "/dev/pts/3", "/dev/pts/4")) {
    $dev = $__dev;
    $CHILD_ERROR = (-r $dev) ? 0 : 1;
    if (-r $dev) {
        $TTY_DEV = $dev;
        $ENV{TTY_DEV} = ${TTY_DEV} if exists $ENV{TTY_DEV};
        $__sh_loop_rc0 = 0; last;
    } else {
        $CHILD_ERROR = 0;
    }
    $__sh_loop_rc0 = $? >> 8;
}
$? = ($__sh_loop_rc0) << 8; $CHILD_ERROR = $? >> 8;
if (!($TTY_DEV ne "")) {
    my $__sh_loop_rc1 = 0;
    for my $__dev ((__sh_glob('/dev/pts/*'))) {
        $dev = $__dev;
        $CHILD_ERROR = (-r $dev) ? 0 : 1;
        if (-r $dev) {
            $TTY_DEV = $dev;
            $ENV{TTY_DEV} = ${TTY_DEV} if exists $ENV{TTY_DEV};
            $__sh_loop_rc1 = 0; last;
        } else {
            $CHILD_ERROR = 0;
        }
        $__sh_loop_rc1 = $? >> 8;
    }
$? = ($__sh_loop_rc1) << 8; $CHILD_ERROR = $? >> 8;
}
print("Using terminal device: " . (((defined($TTY_DEV) && length($TTY_DEV)) ? $TTY_DEV : "NONE")), "\n");  # line 70
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 1: tty (default) — with a real terminal\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
if ($TTY_DEV ne "") {
    {
        open(my $__sh2_is0, '<&', \*STDIN);
        open(STDIN, '<', $TTY_DEV) or die "Cannot read input: $!\n";
        capture("01-default-terminal", "tty");
    if ($__sh2_is0) { open(STDIN, '<&', $__sh2_is0); close($__sh2_is0); }
    }
} else {
    print "  (skipped — no terminal device available)\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    print "\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 2: tty (default) — stdin from /dev/null\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
{
    open(my $__sh2_is0, '<&', \*STDIN);
    open(STDIN, '<', "/dev/null") or die "Cannot read input: $!\n";
    capture("02-not-a-tty", "tty");
if ($__sh2_is0) { open(STDIN, '<&', $__sh2_is0); close($__sh2_is0); }
}
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 3: tty (default) — piped input\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("03-pipe-notty", "bash", "-c", 'echo "dummy" | tty');
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 4: tty -s — with a real terminal (silent)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
if ($TTY_DEV ne "") {
    {
        open(my $__sh2_is0, '<&', \*STDIN);
        open(STDIN, '<', $TTY_DEV) or die "Cannot read input: $!\n";
        capture("04-silent-terminal", "tty", "-s");
    if ($__sh2_is0) { open(STDIN, '<&', $__sh2_is0); close($__sh2_is0); }
    }
} else {
    print "  (skipped — no terminal device available)\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    print "\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 5: tty -s — stdin from /dev/null (silent, not a tty)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
{
    open(my $__sh2_is0, '<&', \*STDIN);
    open(STDIN, '<', "/dev/null") or die "Cannot read input: $!\n";
    capture("05-silent-notty", "tty", "-s");
if ($__sh2_is0) { open(STDIN, '<&', $__sh2_is0); close($__sh2_is0); }
}
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 6: tty -s — piped input (silent, not a tty)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("06-silent-pipe", "bash", "-c", 'echo "dummy" | tty -s');
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 7: tty --silent — long form, with a real terminal\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
if ($TTY_DEV ne "") {
    {
        open(my $__sh2_is0, '<&', \*STDIN);
        open(STDIN, '<', $TTY_DEV) or die "Cannot read input: $!\n";
        capture("07-long-silent", "tty", "--silent");
    if ($__sh2_is0) { open(STDIN, '<&', $__sh2_is0); close($__sh2_is0); }
    }
} else {
    print "  (skipped — no terminal device available)\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    print "\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 8: tty --quiet — long form, with a real terminal\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
if ($TTY_DEV ne "") {
    {
        open(my $__sh2_is0, '<&', \*STDIN);
        open(STDIN, '<', $TTY_DEV) or die "Cannot read input: $!\n";
        capture("08-long-quiet", "tty", "--quiet");
    if ($__sh2_is0) { open(STDIN, '<&', $__sh2_is0); close($__sh2_is0); }
    }
} else {
    print "  (skipped — no terminal device available)\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    print "\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 9: tty --silent — stdin from /dev/null (not a tty)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
{
    open(my $__sh2_is0, '<&', \*STDIN);
    open(STDIN, '<', "/dev/null") or die "Cannot read input: $!\n";
    capture("09-long-silent-notty", "tty", "--silent");
if ($__sh2_is0) { open(STDIN, '<&', $__sh2_is0); close($__sh2_is0); }
}
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 10: tty --quiet — stdin from /dev/null (not a tty)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
{
    open(my $__sh2_is0, '<&', \*STDIN);
    open(STDIN, '<', "/dev/null") or die "Cannot read input: $!\n";
    capture("10-long-quiet-notty", "tty", "--quiet");
if ($__sh2_is0) { open(STDIN, '<&', $__sh2_is0); close($__sh2_is0); }
}
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 11: tty (default) — inheriting stdin from the script\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("11-inherited", "tty");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 12: tty -s — inheriting stdin from the script\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("12-inherited-silent", "tty", "-s");
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 13: tty -s -s — repeated silent flag\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
if ($TTY_DEV ne "") {
    {
        open(my $__sh2_is0, '<&', \*STDIN);
        open(STDIN, '<', $TTY_DEV) or die "Cannot read input: $!\n";
        capture("13-double-silent", "tty", "-s", "-s");
    if ($__sh2_is0) { open(STDIN, '<&', $__sh2_is0); close($__sh2_is0); }
    }
} else {
    print "  (skipped — no terminal device available)\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    print "\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
print "\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "All tty demo sections completed.\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
