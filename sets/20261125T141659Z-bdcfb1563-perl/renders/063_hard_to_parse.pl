#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';
use English qw(-no_match_vars $ERRNO $EVAL_ERROR $INPUT_RECORD_SEPARATOR $OS_ERROR $PROGRAM_NAME);

my $main_exit_code = 0;
our $CHILD_ERROR = 0;

sub __sh_slice {
    # bash's `${arr[@]:offset:length}` semantics, which a Perl range bound
    # does not reproduce:
    #
    #   * a NEGATIVE offset counts back from the last element, and the slice
    #     ends at the last element (not at `$#arr`);
    #   * an offset before the first element yields the EMPTY list;
    #   * a length past the end stops at the last element — a Perl range would
    #     instead pad with undef, which `join` renders as empty fields.
    #
    # So both bounds are clamped against the real element count here.
    my $__av = shift();   # \@name
    my $__off = shift();
    my $__len = shift();
    my $__n = scalar(@$__av);
    my $__start = $__off < 0 ? $__n + $__off : $__off;
    return () if $__start < 0;
    return () if $__start > $__n;
    return @$__av[$__start .. $__n - 1] if !defined($__len);
    return () if $__len <= 0;
    my $__end = $__start + $__len - 1;
    $__end = $__n - 1 if $__end > $__n - 1;
    return @$__av[$__start .. $__end];
}

our $__SH2_AE;

my $threshold;

my $__argc = @ARGV;
our $__sh2_last_bg = '';
my $output = '';
my $result;
my %matrix;
my $key;
my $value;
my %options;
my $i;
my $flags;
my $j;
my $__ps_tmp0;
my $counter;
my $__ps_tmp1;
my $__ps_tmp2;
my $__sh2_tc0;
my $a = '';
my $b = '';
my $c = '';
my $d = '';
my $e = '';
my $f = '';
my $g = '';
my $h = '';
my $k = '';
my $l = '';
my $m = '';
my $n = '';
my $x = '';
my $y = '';
my $z = '';
my @array;
my $index = '';
my @prefix;
my $var = '';
my $default = '';
my $fallback = '';
my %array;
my @default;
my %prefix;
my $file = '';
my @args;
my %args;
my $input_file = '';
my $max_lines = '';
my $line = '';
my $dir = '';
my %files;
my @result;
my %result;
my $matrix;
my $options;
my $array = '';
my $args = '';
my $files = '';

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
# ── pipeline front end (coreutils-ffi-pipeline C ABI) ──────────────
# uu_pipeline_run_str runs a LINEAR pipeline (a|b|c) with redirections
# in-process, so the Perl backend never emits `bash -c` for those shapes.
my $__SH_PIPE = 0;
my $__SH_PIPE_TRIED = 0;
# Shell single-quote a runtime value so the pipeline parser sees one argv.
sub __sh_q {
    my $s = shift;
    $s = '' unless defined $s;
    $s =~ s/'/'\\''/g;
    return "'$s'";
}
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
sub __sh_pipe_run {
    my ($cmd, $opts) = @_;
    # `$$` in rebuilt shell text is the SCRIPT's pid in bash (one shell for
    # the whole program). Each rebuilt command here runs in a FRESH `bash -c`,
    # whose own `$$` would differ per call — so a source `2>/tmp/e.$$` writer
    # and a later `cat /tmp/e.$$` reader (single-quoted by __sh_q, never
    # expanded) could never agree (realpath-cmdsub's stderr capture). Expand
    # to the Perl pid up front so writer, reader and `unlink("…" . ($$))`
    # all name the same file.
    $cmd =~ s/\$\$/$$/g;
    if (__sh_pipe_load() && !__sh_pipe_unsupported_util($cmd)) {
        my $rc = __sh_pipe_run_str($cmd, defined $opts ? $opts : '');
        # `-2` parse / `-3` backend; `127` is the multicall's "unknown util"
        # (not a uutils command) — run those through the shell instead.
        if ($rc >= 0 && $rc != 127) { $CHILD_ERROR = $rc; return $rc; }
    }
    # Not parseable / unknown util / no backend / no .so: run it via the shell.
    # argv0 after the command STRING ($0, not `bash`): a rebuilt command may
    # contain `$0`/`${0}` from the source (`script_dir=$(cd -- "$(dirname
    # "$0")" && pwd)`), and bash passes the script's $0 down to every subshell
    # and pipeline. With argv0 defaulting to "bash", `dirname ${0}` saw "bash"
    # → "." → pwd returned the CWD instead of the script's directory
    # (qx-var-builtin-cd).
    system('bash', '-c', $cmd, $0);
    $CHILD_ERROR = $? >> 8;
    return $CHILD_ERROR;
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

$result = int(((0+(((do { my $__d = (do { my $__d = $f; eval { int(($e)) % ($__d) } }); eval { int(((($a + $b) * ($c - $d))) / ($__d)) } }) + ($g ** $h)) - ((0+$i) << (0+$j)))) | (0+((0+((0+$k) & (0+$l))) ^ (0+((0+$m) | (0+$n)))))));  # line 7
%matrix = ();  # line 10
$matrix{'0,0'} = int((($x + $y) * $z));  # line 11
$matrix{'1,1'} = $array[${index}];  # line 12
$matrix{'2,2'} = @prefix;  # line 13
$matrix{'3,3'} = scalar(@array);  # line 14
$output = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     print("Result: " . (do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     print("Nested: " . (do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     print("Deep: " . (do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     print "Level 4\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
;; } $__cap =~ s/\n+\z//; $__cap }), "\n");
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
;; } $__cap =~ s/\n+\z//; $__cap }), "\n");
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
;; } $__cap =~ s/\n+\z//; $__cap }), "\n");
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
;; } $__cap =~ s/\n+\z//; $__cap };
say ((defined($var) && length($var)) ? $var : ((defined($default) && length($default)) ? $default : ((defined($fallback) && length($fallback)) ? $fallback : "computed")));
say ((defined($array{$index}) && length($array{$index})) ? $array{$index} : join(' ', __sh_slice(\@default, 0, 2)));  # line 21
system('bash', '-c', q['cat' <<'_SH2DOC_' | 'grep' '-v' '^#' | 'sed' 's/^/  /'
# This is a comment
$(echo "Command substitution")
${var:-default}
$(( 1 + 2 * 3 ))
_SH2DOC_]);
$| = 1;  # line 33
my $__sh2_bg2 = fork();
$__sh2_last_bg = $__sh2_bg2;
if (defined $__sh2_bg2 && $__sh2_bg2 == 0) {
    {
        local %ENV = %ENV;
        sleep("1");
        print "Starting\n";
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
            }
    exit $main_exit_code;
}
$| = 1;  # line 34
my $__sh2_bg3 = fork();
$__sh2_last_bg = $__sh2_bg3;
if (defined $__sh2_bg3 && $__sh2_bg3 == 0) {
    {
        local %ENV = %ENV;
        sleep("2");
        print "Processing\n";
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
            }
    exit $main_exit_code;
}
while (wait() != -1) {}  # line 35
print "All done\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
if (($var =~ /^[0-9]+$/) && ($var > 0) && -f $file) {
    if ((join((defined($ENV{IFS}) && length($ENV{IFS})) ? substr($ENV{IFS}, 0, 1) : ' ', @array) =~ /value/) || (scalar(@array) > 5)) {
        if (length("$(echo")) {
            print "Deeply nested condition met\n";
            $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        } else {
                    }
    }
}
if ((do { my $__cap = do { my $__t = $var . "\n"; $__t =~ tr/A-Z/a-z/; $__t }; $__cap =~ s/\n+\z//; $__cap }) =~ /^(?:.*[0-9].*)$/) {  # line 48
    if ((lc($var)) =~ /^(?:.*pattern.*)$/) {
        print "Double nested pattern\n";
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
            } else {
        print "Single nested pattern\n";
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
            }
} else {
    print "No numbers\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    }
sub complex_function {
    local @ARGV = @_;
    my $__argc = @ARGV;
    my @args = (@ARGV);
        my %options = ();
        my $i = "0";
    while (($i < scalar(@args))) {
        if (($args[$i]) =~ /^(?:--.*)$/) {
            my $key = (($args[$i] =~ s{^--}{}r));
                        my $value = ((defined($args[$i+1]) && length($args[$i+1])) ? $args[$i+1] : "true");
                        $options{$key} = $value;
            $i = int($i+=2);
        } elsif (($args[$i]) =~ /^(?:-.*)$/) {
            my $flags = (($args[$i] =~ s{^-}{}r));
                        my $j = "0";
            while (($j < length($flags))) {
                $options{substr($flags, $j, 1)} = "true";
                (($j++));
            }
            (($i++));
        } else {
            last;
        }
    }
    return "Processed " . (join(' ', scalar(keys %options))) . " options";
}
$i = int($i=0);  # line 97
while (($i<scalar(@array))) {
    $j = int($j=0);
    while (($j<scalar(@array[$i]))) {
        if (($array[$i][$j] > $threshold)) {
            $result{'i'} = (do { print STDERR "$0: syntax error in expression (\" result[i] + array[i][j] \")\n"; undef });
        }
        (($j++));
    }
    $? = (((($i++)) != 0) ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;
}
$__ps_tmp0 = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
$ENV{input_file} = $input_file;
$ENV{max_lines} = $max_lines;
$ENV{__ps_tmp0} = $__ps_tmp0;
$? = (__sh_pipe_run(q{'grep' '-v' "^#" } . __sh_q($input_file) . q{ | 'head' '-n' } . __sh_q($max_lines) . ' > ' . __sh_q($__ps_tmp0), "pipefail")) << 8; $CHILD_ERROR = $? >> 8;
{
    open(my $__sh2_is0, '<&', \*STDIN);
    open(STDIN, '<', $__ps_tmp0) or die "Cannot read input: $!\n";
    while (1) {
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        last unless (do { my $__rd = <STDIN>; if (defined $__rd) { chomp $__rd; $line = $__rd; 1 } else { 0 } } && length($line) && ($counter < $max_lines));
        if (($line =~ /^[[:space:]]*#/)) {
            next;
        }
        if (($line) =~ /^(?:.*\$\(.*\).*)$/) {
            say "Contains command substitution: $line";
                    } elsif (($line) =~ /^(?:.*\$\{[^}].*\}.*)$/) {
            say "Contains parameter expansion: $line";
                    } elsif (($line) =~ /^(?:.*\$\(\(.*\)\).*)$/) {
            say "Contains arithmetic expansion: $line";
                    }
        (($counter++));
    }
if ($__sh2_is0) { open(STDIN, '<&', $__sh2_is0); close($__sh2_is0); }
}
do { unlink($__ps_tmp0); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
$result = ((do { no warnings; my $__t = (((defined($var) && length($var)) ? $var : "0")); !(defined($__t) && length($__t)) && ($__SH2_AE = 1); $__t }) + (do { no warnings; my $__t = (((defined($array{((defined($index) && length($index)) ? $index : "0")}) && length($array{((defined($index) && length($index)) ? $index : "0")})) ? $array{((defined($index) && length($index)) ? $index : "0")} : "0")); !(defined($__t) && length($__t)) && ($__SH2_AE = 1); $__t }));
print '2', "\n"; $? = 0 << 8; $CHILD_ERROR = $? >> 8;$__ps_tmp1 = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
do { my @__sl = do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__fh, '<', 'file1.txt') or do { $CHILD_ERROR = 2; () }; my $__c = <$__fh> // ''; close $__fh; split(/\n/, $__c) }; pop @__sl if @__sl && $__sl[-1] eq q{}; @__sl = sort { our ($a, $b); $a cmp $b } @__sl; open(my $__ofh, '>', $__ps_tmp1) or die "sort: $__ps_tmp1: $ERRNO"; if (@__sl) { print $__ofh join("\n", @__sl), "\n"; } close $__ofh; }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;$__ps_tmp2 = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
do { my @__sl = do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__fh, '<', 'file2.txt') or do { $CHILD_ERROR = 2; () }; my $__c = <$__fh> // ''; close $__fh; split(/\n/, $__c) }; pop @__sl if @__sl && $__sl[-1] eq q{}; @__sl = sort { our ($a, $b); $a cmp $b } @__sl; open(my $__ofh, '>', $__ps_tmp2) or die "sort: $__ps_tmp2: $ERRNO"; if (@__sl) { print $__ofh join("\n", @__sl), "\n"; } close $__ofh; }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;open(my $__sh2_rfh1, '>&', \*STDOUT) or die "Cannot dup stdout: $!\n";
close(STDOUT); open(STDOUT, '>', "comparison.txt") or die "Cannot write to comparison.txt: $!\n";
my $__sh2_rsel1 = select(STDOUT);
my $__rc = 0; do { my @__a = split(/\n/, do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__f, '<', $__ps_tmp1) or die "diff: $__ps_tmp1: $ERRNO"; my $__c = <$__f>; close $__f; $__c }, -1); pop @__a if @__a && $__a[-1] eq q{}; my @__b = split(/\n/, do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__g, '<', $__ps_tmp2) or die "diff: $__ps_tmp2: $ERRNO"; my $__c = <$__g>; close $__g; $__c }, -1); pop @__b if @__b && $__b[-1] eq q{}; my @__dp; for my $__i (0..$#__a) { for my $__j (0..$#__b) { if ($__a[$__i] eq $__b[$__j]) { $__dp[$__i+1][$__j+1] = $__dp[$__i][$__j] + 1; } else { $__dp[$__i+1][$__j+1] = ($__dp[$__i][$__j+1] > $__dp[$__i+1][$__j]) ? $__dp[$__i][$__j+1] : $__dp[$__i+1][$__j]; } } } my ($__i, $__j) = ($#__a, $#__b); my @__ops; while ($__i >= 0 && $__j >= 0) { if ($__a[$__i] eq $__b[$__j]) { unshift @__ops, [0, $__i, $__j]; $__i--; $__j--; } elsif ($__dp[$__i][$__j+1] >= $__dp[$__i+1][$__j]) { unshift @__ops, [-1, $__i, $__j]; $__i--; } else { unshift @__ops, [1, $__i, $__j]; $__j--; } } while ($__i >= 0) { unshift @__ops, [-1, $__i, -1]; $__i--; } while ($__j >= 0) { unshift @__ops, [1, -1, $__j]; $__j--; }  my $__is_out = 0; my $__fh = \*STDOUT; my $__changed = 0; my $__idx = 0; my $__common = -1; while ($__idx < @__ops) { if ($__ops[$__idx][0] == 0) { $__common = $__ops[$__idx][1] + 1; $__idx++; next; } my @__dels; my @__adds; my $__last_a = -1; my $__last_b = -1; while ($__idx < @__ops && $__ops[$__idx][0] != 0) { if ($__ops[$__idx][0] == -1) { push @__dels, $__ops[$__idx][1]; $__last_a = $__ops[$__idx][1]; } else { push @__adds, $__ops[$__idx][2]; $__last_b = $__ops[$__idx][2]; } $__idx++; } $__changed = 1; my $__a1 = $__dels[0] + 1; my $__a2 = $__dels[-1] + 1; my $__b1 = $__adds[0] + 1; my $__b2 = $__adds[-1] + 1; my $__astr = $__a1 == $__a2 ? "$__a1" : "$__a1,$__a2"; my $__bstr = $__b1 == $__b2 ? "$__b1" : "$__b1,$__b2"; my $__opch = (@__dels && @__adds) ? "c" : (@__dels ? "d" : "a"); my $__hdr; if ($__opch eq "c") { $__hdr = "$__astr$__opch$__bstr"; } elsif ($__opch eq "d") { $__hdr = "$__astr$__opch" . (defined $__last_b ? $__last_b + 1 : 1); } else { my $__apos = $__common; $__hdr = "$__apos$__opch$__bstr"; } print $__fh "$__hdr\n"; for my $__k (@__dels) { print $__fh "< $__a[$__k]\n"; } if (@__dels && @__adds) { print $__fh "---\n"; } for my $__k (@__adds) { print $__fh "> $__b[$__k]\n"; } } $__rc = $__changed ? 1 : 0; close $__fh if $__is_out; }; $? = ($__rc) << 8; $CHILD_ERROR = $? >> 8; select($__sh2_rsel1); open(STDOUT, '>&', $__sh2_rfh1) or die "Cannot restore stdout: $!\n"; close($__sh2_rfh1);
do { unlink($__ps_tmp1, $__ps_tmp2); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
if (((length($var) && -f $file) || -d $dir) && length("$(wc -l <")) {
    print "Complex test passed\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
print "a1x a1y a1z a2x a2y a2z a3x a3y a3z b1x b1y b1z b2x b2y b2z b3x b3y b3z c1x c1y c1z c2x c2y c2z c3x c3y c3z\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$ENV{var} = $var;  # line 161
system('bash', '-c', q|'tr' '[:upper:]' '[:lower:]' <<< $('echo' "UPPER: ${var^^}")|);
say complex_function("--long-option=value with spaces", "--array-option", "item1", "item2", "item3", "--flag", "positional argument", ((defined($var) && length($var)) ? $var : "default"), "computed");
{  # line 173
    my $__sh2_pcap = '';
    { local *STDOUT; open(STDOUT, '>', \$__sh2_pcap) or die "capture: $!\n";
        {
            local %ENV = %ENV;
            my $__save___sh2_tc0 = $__sh2_tc0;
            my $__save_i = $i;
            my $__save_result = %result;
            $__sh2_tc0 = do { my $__cap = do { my $__t = $var . "\n"; $__t =~ tr/A-Z/a-z/; $__t }; $__cap =~ s/\n+\z//; $__cap };
            if (($__sh2_tc0 =~ /^[a-z]+$/)) {
                $i = int($i=0);
                while (($i<scalar(@array))) {
                    if (($array[$i] > $threshold) && -f $files{'i'}) {
                        $result{'i'} = (${result}[$i] + (do { no warnings; my $__t = (do { no strict 'vars'; no warnings; local $ENV{files} = $files; __sh_pipe_capture('wc -l < ' . __sh_q(${files}{i}), "pipefail") }); !(defined($__t) && length($__t)) && ($__SH2_AE = 1); $__t }));
                    }
                    $? = (((($i++)) != 0) ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;
                }
            }
            $__sh2_tc0 = $__save___sh2_tc0;
            $i = $__save_i;
            %result = $__save_result;
        }
    ; }
    my $__sh2_ptail = q{'sort' '-n' | 'tail' '-n' '5' > 'final_result.txt'};
    open(my $__sh2_pfh, '|-', 'bash', '-c', $__sh2_ptail, $0) or die "sh2perl: pipe open failed: $!\n";
    print $__sh2_pfh $__sh2_pcap;
    close $__sh2_pfh;
    $? = ($? >> 8) << 8; $CHILD_ERROR = $? >> 8;
}

exit $? >> 8;
