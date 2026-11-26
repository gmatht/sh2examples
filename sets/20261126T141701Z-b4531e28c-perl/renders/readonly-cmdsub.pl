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
    $tmp_stdout = do { my $_r = __sh_uu_capture('mktemp', '/tmp/readonly_demo_stdout_XXXXXX'); $CHILD_ERROR = $__sh_uu_rc; $_r =~ s/\n+\z//; $_r; };
    $tmp_stderr = do { my $_r = __sh_uu_capture('mktemp', '/tmp/readonly_demo_stderr_XXXXXX'); $CHILD_ERROR = $__sh_uu_rc; $_r =~ s/\n+\z//; $_r; };
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
print " SECTION 1: Basic readonly variable\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("01-readonly-var", "bash", "-c", '
        readonly MY_VAR="hello_world"
        echo "MY_VAR=${MY_VAR}"
        # Attempt to change it (will fail, but we capture)
        MY_VAR="changed" 2>/dev/null || echo "Assignment denied: $?"
        echo "MY_VAR after attempted change=${MY_VAR}"
    ');$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 2: readonly -p  (print readonly variables)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("02-readonly-p", "bash", "-c", '
        readonly FOO=alpha
        readonly BAR=beta
        readonly -p | head -5
    ');$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 3: readonly -f  (readonly functions)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("03-readonly-f", "bash", "-c", '
        myfunc() { echo "Hello from myfunc"; }
        readonly -f myfunc
        myfunc
        # Attempt to unset the function (will fail)
        unset -f myfunc 2>/dev/null || echo "unset -f denied"
        # Verify function still exists
        myfunc
    ');$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 4: readonly -a  (indexed array)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("04-readonly-a", "bash", "-c", '
        declare -a MY_ARR=(zero one two)
        readonly -a MY_ARR
        echo "MY_ARR[0]=${MY_ARR[0]}"
        echo "MY_ARR[1]=${MY_ARR[1]}"
        # Attempt to change an element (will fail)
        MY_ARR[0]=ZERO 2>/dev/null || echo "Array assignment denied"
        echo "MY_ARR[0] after attempt=${MY_ARR[0]}"
    ');$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 5: readonly -A  (associative array)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("05-readonly-A", "bash", "-c", '
        declare -A MY_ASSOC=([key1]=val1 [key2]=val2)
        readonly -A MY_ASSOC
        echo "MY_ASSOC[key1]=${MY_ASSOC[key1]}"
        echo "MY_ASSOC[key2]=${MY_ASSOC[key2]}"
        # Attempt to change (will fail)
        MY_ASSOC[key1]=CHANGED 2>/dev/null || echo "Assoc assignment denied"
        echo "MY_ASSOC[key1] after attempt=${MY_ASSOC[key1]}"
    ');$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 6: readonly --  (no more options)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("06a-readonly-dash-ok", "bash", "-c", '
        readonly -- MYVAR=hello
        echo "MYVAR=${MYVAR}"
        # Confirm it is indeed readonly
        MYVAR=world 2>/dev/null || echo "Assignment denied (expected)"
    ');$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("06b-readonly-dash-invalid", "bash", "-c", '
        readonly -- -notvalid 2>&1 || true
    ');$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 7: Invalid variable name (error case)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("07-invalid-name", "bash", "-c", '
        readonly 123invalid 2>&1 || true
    ');$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print " SECTION 8: readonly -f -p  (list readonly functions)\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "============================================================\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
capture("08-readonly-f-p", "bash", "-c", '
        f1() { :; }
        f2() { :; }
        readonly -f f1 f2
        readonly -f -p | head -5
    ');$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "All readonly demo sections completed.\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
exit "0";

exit $? >> 8;
