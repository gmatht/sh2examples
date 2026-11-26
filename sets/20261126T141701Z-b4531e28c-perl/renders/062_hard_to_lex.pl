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
my $result;
my $complex_var;
my $param1;
my $param2;
my $param3;
my $message;
my @array;
my $var1;
my $var2;
my $var3;
my $array;

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
# ── pipeline front end (coreutils-ffi-pipeline C ABI) ──────────────
# uu_pipeline_run_str runs a LINEAR pipeline (a|b|c) with redirections
# in-process, so the Perl backend never emits `bash -c` for those shapes.
my $__SH_PIPE = 0;
my $__SH_PIPE_TRIED = 0;
# Shell single-quote a runtime value so the pipeline parser sees one argv.
sub __sh_pipe_load {
    return 1 if $__SH_PIPE;
    return 0 if $__SH_PIPE_TRIED;
    $__SH_PIPE_TRIED = 1;
    my @cand;
    push @cand, $ENV{SH2_UU_PIPELINE_LIB} if defined $ENV{SH2_UU_PIPELINE_LIB};
    for my $up ('', '../', '../../', '../../../', '../../../../', '../../../../../') {
        push @cand, "${up}runtime/lib/libcoreutils_ffi_pipeline.so";
    }
    push @cand, 'libcoreutils_ffi_pipeline.so';
    for my $lib (@cand) {
        next unless defined $lib && -e $lib;
        my $ok = eval {
            require FFI::Platypus;
            my $ffi = FFI::Platypus->new(api => 2);
            $ffi->lib($lib);
            $ffi->attach([uu_pipeline_run_str => '__sh_pipe_run_str'] => ['string','string'] => 'int');
            $ffi->attach([uu_pipeline_last_code => '__sh_pipe_last_code'] => [] => 'int');
            $__SH_PIPE = 1;
            1;
        };
        last if $ok;
    }
    # Point the process backends at the multicall helper shipped alongside.
    if ($__SH_PIPE && !defined $ENV{UU_FFI_MULTICALL}) {
        for my $up ('', '../', '../../', '../../../') {
            my $m = "${up}runtime/lib/coreutils-ffi-multicall";
            if (-x $m) { $ENV{UU_FFI_MULTICALL} = $m; last; }
        }
    }
    $__SH_PIPE;
}
sub __sh_pipe_unsupported_util {
    my ($cmd) = @_;
    # The reconstructed command quotes each stage's util name. These are not
    # in the pipeline .so (non-uutils tools / shell builtins); routing them
    # in-process is a race between rc 127 (unknown util) and rc 141 (SIGPIPE
    # from the neighbour writing into the dead stage) — 141 is a real exit
    # code, so the caller skipped the shell fallback and printed nothing
    # (parse-sq-with-for, intermittently). Send them to the shell up front.
    return $cmd =~ m{(?:^|\|)\s*'(?:perl|python3?|bc|find|strings|gunzip|gzip|zstd|bzip2|xz|tar|unzip|mapfile|readarray|ruby|node|php)'};
}
sub __sh_pipe_capture {
    my ($cmd, $opts) = @_;
    # See __sh_pipe_run: one Perl pid for writer and reader of `…$$`.
    $cmd =~ s/\$\$/$$/g;
    if (__sh_pipe_load() && !__sh_pipe_unsupported_util($cmd)) {
        my $tmp = "/tmp/sh2_pipe_${$}_" . ($__SH_UU_SEQ++) . ".cap";
        my $rc = __sh_pipe_run_str($cmd . ' > ' . $tmp, defined $opts ? $opts : '');
        if ($rc >= 0 && $rc != 127) {
            my $out = '';
            if (open(my $rf, '<', $tmp)) { $out = do { local $/; <$rf> } // ''; close $rf; }
            unlink $tmp;
            $CHILD_ERROR = $rc;
            $out =~ s/\n+\z//;
            return $out;
        }
        unlink $tmp;
    }
    my $out = do {
        open(my $fh, '-|', 'bash', '-c', $cmd, $0) or do { $CHILD_ERROR = 127; return ''; };
        my $r = do { local $/; <$fh> };
        close $fh;
        $r;
    };
    $CHILD_ERROR = $? >> 8;
    $out =~ s/\n+\z//;
    return $out // '';
}
# __SH_RUN_EXT_END

print "Testing ambiguous operators...\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$result = int(512);
say "2**3**2 = $result";  # line 10
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "Testing complex parameter expansions...\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$complex_var = "hello world";
say (($complex_var =~ s{^.*?o}{}r));  # line 15
say (($complex_var =~ s{^.*o}{}r));  # line 16
say (($complex_var =~ s{o[^o]*$}{}r));  # line 17
say (($complex_var =~ s{o.*$}{}r));  # line 18
print "Testing complex here-documents...\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print('This is a test line
This is not a test line
This is another test line
');
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "Testing nested arithmetic...\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$result = int(40);
say "Complex arithmetic: $result";  # line 31
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "Testing nested command substitution...\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print("Current dir: " . (((defined($ENV{PWD}) && length($ENV{PWD})) ? $ENV{PWD} : do { my $__sh_cap; $__sh_cap = (require Cwd, Cwd::getcwd()); $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap })), "\n");  # line 35
print("User: " . (((defined($ENV{USER}) && length($ENV{USER})) ? $ENV{USER} : __sh_pipe_capture('whoami', "pipefail"))), "\n");  # line 36
print "Testing process substitution...\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "Testing complex brace expansion...\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "a1x a1y a1z a2x a2y a2z a3x a3y a3z b1x b1y b1z b2x b2y b2z b3x b3y b3z c1x c1y c1z c2x c2y c2z c3x c3y c3z\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "Testing simple case patterns...\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
if (($__sh_args[0]) =~ /^(?:test)$/) {  # line 48
    print "Matched test\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } else {
    print "Default case\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    }
sub complex_function {
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    my $param1 = $__sh_args[0];
        my $param2 = ((defined($__sh_args[1]) && length($__sh_args[1])) ? $__sh_args[1] : "default");
        my $param3 = (($__sh_args[2] =~ s/"/\\"/gr));
        say "Param1: $param1";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    say "Param2: $param2";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    say "Param3: $param3";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    my $result = join(' ', split(/\s+/, do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n"; do { my $__x = $param1; $__x =~ s{old}{new}g; print STDOUT $__x, "\n"; }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;;; } $__cap =~ s/\n+\z//; $__cap }));
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    say "Result: $result";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
print "Testing simple pipeline...\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
my $__sh_pv = do { my $__capf = "/tmp/sh2_cap_3_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { do { my $__g = do { my $__capf = "/tmp/sh2_cap_2_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { __sh_uu_run('ls', '-l', '-a');$? = ($__sh_uu_rc) << 8; 
 ; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }; my $__m = 0; for my $__l (split(/\n/, $__g)) { if ($__l =~ /^d/) { print $__l, "\n"; $__m = 1; } }  }; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }; do { my @__pl = split(/\n/, $__sh_pv); pop @__pl if @__pl && $__pl[-1] eq ''; @__pl = @__pl[0..4] if @__pl > 5; say join("\n", @__pl) if @__pl;  };
print "Testing mixed arithmetic...\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$result = int(776);
say "Mixed base result: $result";  # line 82
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "Testing complex string interpolation...\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$message = "Hello, " . (((defined($ENV{USER}) && length($ENV{USER})) ? $ENV{USER} : __sh_pipe_capture('whoami', "pipefail"))) . "! Your home is " . (((defined($ENV{HOME}) && length($ENV{HOME})) ? $ENV{HOME} : do { my $_r = __sh_uu_capture('echo', '~'); $CHILD_ERROR = $__sh_uu_rc; $_r =~ s/\n+\z//; $_r; }));  # line 86
say $message;  # line 87
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "Testing simple test expressions...\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
if (-f 'file.txt') {
    print "File exists\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    print "File does not exist\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
print "Testing complex array operations...\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
@array = ("item1", "item2", "item3");
push @array, ("item4");
print("Array: " . (join(' ', @array)), "\n");  # line 101
print("Length: " . (join(' ', scalar(@array))), "\n");
print("First item: " . ($array[0]), "\n");  # line 103
sub test_locals {  # line 106
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    my $var1 = $__sh_args[0];
        my $var2 = ((defined($__sh_args[1]) && length($__sh_args[1])) ? $__sh_args[1] : "default_value");
        my $var3 = do { my $__cap = do { my $__t = $var1 . "\n"; $__t =~ tr/a-z/A-Z/; $__t }; $__cap =~ s/\n+\z//; $__cap };
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    say "Var1: $var1";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    say "Var2: $var2";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    say "Var3: $var3";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
complex_function("test\"quote", "second_param", "third\"param");
test_locals("hello", "world");$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
