#!/usr/bin/env perl
use strict;

sub __sh_glob {
    my @__g = glob($_[0]);
    return @__g ? @__g : ($_[0]);
}

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

my $PWD;
my $USER;

our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
use feature 'say';
use Sys::Hostname qw(hostname);
use English qw(-no_match_vars $ERRNO $EVAL_ERROR $INPUT_RECORD_SEPARATOR $OS_ERROR $PROGRAM_NAME);
$ENV{HOSTNAME} //= hostname();

our $CHILD_ERROR = 0;
my $__nocasematch = 0;
my $__ps_tmp1;
my $__ps_tmp0;
my $__ps_tmp3;
my $__ps_tmp2;
my $name;
my $i;
my $j;
my %config;
my $__ps_tmp4;
my $__ps_tmp5;
my $filename;
my $basename;
my $extension;
my $uppercase;
my $result;
my @numbers;
my @middle;
my @first_half;
my @last_half;
my $file;
my $message;
my $input;
my $sanitized;
my $a = '';
my $b = '';
my $user = '';
my $pass = '';
my $uid = '';
my $gid = '';
my $info = '';
my $home = '';
my $shell = '';
my $config;

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

$__ps_tmp1 = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
open(my $__sh2_rfh2, '>&', \*STDOUT) or die "Cannot dup stdout: $!\n";
close(STDOUT); open(STDOUT, '>', $__ps_tmp1) or die "Cannot write to __ps_tmp1: $!\n";
my $__sh2_rsel2 = select(STDOUT);
$__ps_tmp0 = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
$ENV{__ps_tmp0} = $__ps_tmp0;
__sh_pipe_run(q{'grep' '-v' "^#" '/etc/passwd' | 'cut' '-d' ':' '-f' '1' > } . __sh_q($__ps_tmp0), "pipefail");
__sh_uu_run('sort', $__ps_tmp0);
do { unlink($__ps_tmp0); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
select($__sh2_rsel2); open(STDOUT, '>&', $__sh2_rfh2) or die "Cannot restore stdout: $!\n"; close($__sh2_rfh2);
$__ps_tmp3 = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
open(my $__sh2_rfh3, '>&', \*STDOUT) or die "Cannot dup stdout: $!\n";
close(STDOUT); open(STDOUT, '>', $__ps_tmp3) or die "Cannot write to __ps_tmp3: $!\n";
my $__sh2_rsel3 = select(STDOUT);
$__ps_tmp2 = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
$ENV{__ps_tmp2} = $__ps_tmp2;
__sh_pipe_run(q{'grep' '-v' "^#" '/etc/group' | 'cut' '-d' ':' '-f' '1' > } . __sh_q($__ps_tmp2), "pipefail");
__sh_uu_run('sort', $__ps_tmp2);
do { unlink($__ps_tmp2); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
select($__sh2_rsel3); open(STDOUT, '>&', $__sh2_rfh3) or die "Cannot restore stdout: $!\n"; close($__sh2_rfh3);
my $__rc = 0; do { my @__a = split(/\n/, do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__f, '<', $__ps_tmp1) or die "diff: $__ps_tmp1: $ERRNO"; my $__c = <$__f>; close $__f; $__c }, -1); pop @__a if @__a && $__a[-1] eq q{}; my @__b = split(/\n/, do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__g, '<', $__ps_tmp3) or die "diff: $__ps_tmp3: $ERRNO"; my $__c = <$__g>; close $__g; $__c }, -1); pop @__b if @__b && $__b[-1] eq q{}; my @__dp; for my $__i (0..$#__a) { for my $__j (0..$#__b) { if ($__a[$__i] eq $__b[$__j]) { $__dp[$__i+1][$__j+1] = $__dp[$__i][$__j] + 1; } else { $__dp[$__i+1][$__j+1] = ($__dp[$__i][$__j+1] > $__dp[$__i+1][$__j]) ? $__dp[$__i][$__j+1] : $__dp[$__i+1][$__j]; } } } my ($__i, $__j) = ($#__a, $#__b); my @__ops; while ($__i >= 0 && $__j >= 0) { if ($__a[$__i] eq $__b[$__j]) { unshift @__ops, [0, $__i, $__j]; $__i--; $__j--; } elsif ($__dp[$__i][$__j+1] >= $__dp[$__i+1][$__j]) { unshift @__ops, [-1, $__i, $__j]; $__i--; } else { unshift @__ops, [1, $__i, $__j]; $__j--; } } while ($__i >= 0) { unshift @__ops, [-1, $__i, -1]; $__i--; } while ($__j >= 0) { unshift @__ops, [1, -1, $__j]; $__j--; }  my $__is_out = 0; my $__fh = \*STDOUT; my $__changed = 0; my $__idx = 0; my $__common = -1; while ($__idx < @__ops) { if ($__ops[$__idx][0] == 0) { $__common = $__ops[$__idx][1] + 1; $__idx++; next; } my @__dels; my @__adds; my $__last_a = -1; my $__last_b = -1; while ($__idx < @__ops && $__ops[$__idx][0] != 0) { if ($__ops[$__idx][0] == -1) { push @__dels, $__ops[$__idx][1]; $__last_a = $__ops[$__idx][1]; } else { push @__adds, $__ops[$__idx][2]; $__last_b = $__ops[$__idx][2]; } $__idx++; } $__changed = 1; my $__a1 = $__dels[0] + 1; my $__a2 = $__dels[-1] + 1; my $__b1 = $__adds[0] + 1; my $__b2 = $__adds[-1] + 1; my $__astr = $__a1 == $__a2 ? "$__a1" : "$__a1,$__a2"; my $__bstr = $__b1 == $__b2 ? "$__b1" : "$__b1,$__b2"; my $__opch = (@__dels && @__adds) ? "c" : (@__dels ? "d" : "a"); my $__hdr; if ($__opch eq "c") { $__hdr = "$__astr$__opch$__bstr"; } elsif ($__opch eq "d") { $__hdr = "$__astr$__opch" . (defined $__last_b ? $__last_b + 1 : 1); } else { my $__apos = $__common; $__hdr = "$__apos$__opch$__bstr"; } print $__fh "$__hdr\n"; for my $__k (@__dels) { print $__fh "< $__a[$__k]\n"; } if (@__dels && @__adds) { print $__fh "---\n"; } for my $__k (@__adds) { print $__fh "> $__b[$__k]\n"; } } $__rc = $__changed ? 1 : 0; close $__fh if $__is_out; }; $? = ($__rc) << 8; $CHILD_ERROR = $? >> 8; do { unlink($__ps_tmp1, $__ps_tmp3); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
print "Files:  file_a_1..10.txt file_a_1..10.log file_a_1..10.dat file_a_20.txt file_a_20.log file_a_20.dat file_a_30..40.txt file_a_30..40.log file_a_30..40.dat file_b_1..10.txt file_b_1..10.log file_b_1..10.dat file_b_20.txt file_b_20.log file_b_20.dat file_b_30..40.txt file_b_30..40.log file_b_30..40.dat file_c_1..10.txt file_c_1..10.log file_c_1..10.dat file_c_20.txt file_c_20.log file_c_20.dat file_c_30..40.txt file_c_30..40.log file_c_30..40.dat file_d_1..10.txt file_d_1..10.log file_d_1..10.dat file_d_20.txt file_d_20.log file_d_20.dat file_d_30..40.txt file_d_30..40.log file_d_30..40.dat file_e_1..10.txt file_e_1..10.log file_e_1..10.dat file_e_20.txt file_e_20.log file_e_20.dat file_e_30..40.txt file_e_30..40.log file_e_30..40.dat file_f_1..10.txt file_f_1..10.log file_f_1..10.dat file_f_20.txt file_f_20.log file_f_20.dat file_f_30..40.txt file_f_30..40.log file_f_30..40.dat file_g_1..10.txt file_g_1..10.log file_g_1..10.dat file_g_20.txt file_g_20.log file_g_20.dat file_g_30..40.txt file_g_30..40.log file_g_30..40.dat file_h_1..10.txt file_h_1..10.log file_h_1..10.dat file_h_20.txt file_h_20.log file_h_20.dat file_h_30..40.txt file_h_30..40.log file_h_30..40.dat file_i_1..10.txt file_i_1..10.log file_i_1..10.dat file_i_20.txt file_i_20.log file_i_20.dat file_i_30..40.txt file_i_30..40.log file_i_30..40.dat file_j_1..10.txt file_j_1..10.log file_j_1..10.dat file_j_20.txt file_j_20.log file_j_20.dat file_j_30..40.txt file_j_30..40.log file_j_30..40.dat file_k_1..10.txt file_k_1..10.log file_k_1..10.dat file_k_20.txt file_k_20.log file_k_20.dat file_k_30..40.txt file_k_30..40.log file_k_30..40.dat file_l_1..10.txt file_l_1..10.log file_l_1..10.dat file_l_20.txt file_l_20.log file_l_20.dat file_l_30..40.txt file_l_30..40.log file_l_30..40.dat file_m_1..10.txt file_m_1..10.log file_m_1..10.dat file_m_20.txt file_m_20.log file_m_20.dat file_m_30..40.txt file_m_30..40.log file_m_30..40.dat file_n_1..10.txt file_n_1..10.log file_n_1..10.dat file_n_20.txt file_n_20.log file_n_20.dat file_n_30..40.txt file_n_30..40.log file_n_30..40.dat file_o_1..10.txt file_o_1..10.log file_o_1..10.dat file_o_20.txt file_o_20.log file_o_20.dat file_o_30..40.txt file_o_30..40.log file_o_30..40.dat file_p_1..10.txt file_p_1..10.log file_p_1..10.dat file_p_20.txt file_p_20.log file_p_20.dat file_p_30..40.txt file_p_30..40.log file_p_30..40.dat file_q_1..10.txt file_q_1..10.log file_q_1..10.dat file_q_20.txt file_q_20.log file_q_20.dat file_q_30..40.txt file_q_30..40.log file_q_30..40.dat file_r_1..10.txt file_r_1..10.log file_r_1..10.dat file_r_20.txt file_r_20.log file_r_20.dat file_r_30..40.txt file_r_30..40.log file_r_30..40.dat file_s_1..10.txt file_s_1..10.log file_s_1..10.dat file_s_20.txt file_s_20.log file_s_20.dat file_s_30..40.txt file_s_30..40.log file_s_30..40.dat file_t_1..10.txt file_t_1..10.log file_t_1..10.dat file_t_20.txt file_t_20.log file_t_20.dat file_t_30..40.txt file_t_30..40.log file_t_30..40.dat file_u_1..10.txt file_u_1..10.log file_u_1..10.dat file_u_20.txt file_u_20.log file_u_20.dat file_u_30..40.txt file_u_30..40.log file_u_30..40.dat file_v_1..10.txt file_v_1..10.log file_v_1..10.dat file_v_20.txt file_v_20.log file_v_20.dat file_v_30..40.txt file_v_30..40.log file_v_30..40.dat file_w_1..10.txt file_w_1..10.log file_w_1..10.dat file_w_20.txt file_w_20.log file_w_20.dat file_w_30..40.txt file_w_30..40.log file_w_30..40.dat file_x_1..10.txt file_x_1..10.log file_x_1..10.dat file_x_20.txt file_x_20.log file_x_20.dat file_x_30..40.txt file_x_30..40.log file_x_30..40.dat file_y_1..10.txt file_y_1..10.log file_y_1..10.dat file_y_20.txt file_y_20.log file_y_20.dat file_y_30..40.txt file_y_30..40.log file_y_30..40.dat file_z_1..10.txt file_z_1..10.log file_z_1..10.dat file_z_20.txt file_z_20.log file_z_20.dat file_z_30..40.txt file_z_30..40.log file_z_30..40.dat\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$name = "John Doe";
say "Hello John_Doe";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "Length: 8";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print("First: " . (join(' ', "John")), "\n");
print("Last: " . (join(' ', substr($name, -3))), "\n");  # line 17
$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 20
$__nocasematch = 1;$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 21
if (($__sh_args[0]) =~ /^(?:[a-z].*)$/) {  # line 24
    print "Lowercase start\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } elsif (($__sh_args[0]) =~ /^(?:[A-Z].*)$/) {
    print "Uppercase start\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } elsif (($__sh_args[0]) =~ /^(?:[0-9].*)$/) {
    print "Number start\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } elsif (($__sh_args[0]) =~ /^(?:.)$/) {
    print "Single character\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } else {
    print "Something else\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    }
$i = int(2);
$j = (($i++) + (++$i));  # line 34
say "i=$i, j=$j";  # line 35
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
%config = ();  # line 38
$config{'user'} = "admin";
$config{'host'} = "localhost";
$config{'port'} = "8080";
open my $__fh, '>', "config.txt" or die "Cannot write file: $!\n";  # line 44
print {$__fh} "User: \$USER\nHost: \${HOSTNAME:-localhost}\nPath: \$PWD\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$__ps_tmp4 = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
$ENV{__ps_tmp4} = $__ps_tmp4;
__sh_pipe_run(q{'cut' '-d' ':' '-f' '1' '/etc/passwd' | 'sort' > } . __sh_q($__ps_tmp4), "pipefail");
$__ps_tmp5 = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
$ENV{__ps_tmp5} = $__ps_tmp5;
__sh_pipe_run(q{'cut' '-d' ':' '-f' '3' '/etc/passwd' | 'sort' '-n' > } . __sh_q($__ps_tmp5), "pipefail");
do { my @__pl = split(/\n/, do { my $__capf = "/tmp/sh2_cap_1_$$"; { open(my $__capso, '>&', \*STDOUT) or die "capture: $!\n"; close(STDOUT); open(STDOUT, '>', $__capf) or die "capture: $!\n"; my $__capold = select(STDOUT); { __sh_uu_run('paste', $__ps_tmp4, $__ps_tmp5);$? = ($__sh_uu_rc) << 8; 
 ; } select($__capold); open(STDOUT, '>&', $__capso) or die "capture: $!\n"; close($__capso); } my $__cap = ''; if (open(my $__capfh, '<', $__capf)) { $__cap = do { local $/; <$__capfh> } // ''; close $__capfh; } unlink $__capf; $__cap =~ s/\n+\z//; $__cap }); pop @__pl if @__pl && $__pl[-1] eq ''; @__pl = @__pl[0..9] if @__pl > 10; say join("\n", @__pl) if @__pl;  };
do { unlink($__ps_tmp4, $__ps_tmp5); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
if (($__sh_args[0] =~ /^[0-9]+$/) && ($__sh_args[1] eq 'test' || $__sh_args[1] eq 'debug')) {
    print "Valid input\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
do { require File::Path; File::Path::make_path('project/src/main/java', 'project/src/main/resources', 'project/src/test/java', 'project/src/test/resources', 'project/docs/api', 'project/docs/user', 'project/build/classes', 'project/build/lib'); my $__ok = 1; $? = ($__ok ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; };
$filename = "my_file.txt";
$basename = (($filename =~ s{\.[^.]*$}{}r));  # line 79
$extension = (($filename =~ s{^.*\.}{}r));  # line 80
$uppercase = "MY_FILE.TXT";
$result = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     $__SH2_AE = 0;  # line 85
    do { my @__sh_ae = (((do { no warnings; my $__t = (__sh_pipe_capture('wc -l < /etc/passwd', "pipefail")); !(defined($__t) && length($__t)) && ($__SH2_AE = 1); $__t }) + (do { no warnings; my $__t = (__sh_pipe_capture('wc -l < /etc/group', "pipefail")); !(defined($__t) && length($__t)) && ($__SH2_AE = 1); $__t }))); if ($__SH2_AE) {$? = 1 << 8; $CHILD_ERROR = $? >> 8; } else { say $__sh_ae[0];$? = 0 << 8; $CHILD_ERROR = $? >> 8; } };
;; } $__cap =~ s/\n+\z//; $__cap };
do { my @__lp = do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__f, '<', '/etc/passwd') or do { $CHILD_ERROR = 2; () }; my $__c = <$__f> // ''; close $__f; split(/\n/, $__c) }; pop @__lp if @__lp && $__lp[-1] eq q{}; @__lp = grep { !/^#/ } @__lp; @__lp = map { my @__f = split(':', $_); join(':', @__f[0,2]) } @__lp; @__lp = sort { (split(':', $a))[1] <=> (split(':', $b))[1] } @__lp; if (@__lp > 5) { splice(@__lp, 5); } if (@__lp) { open(my $__ofh, '>', 'users.txt') or die "grep: 'users.txt': $ERRNO"; print $__ofh join("\n", @__lp), "\n"; close $__ofh; } }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;{
    open(my $__sh2_is0, '<&', \*STDIN);
    open(STDIN, '<', "/etc/passwd") or die "Cannot read input: $!\n";
    while (do { my $__rd = <STDIN>; if (defined $__rd) { chomp $__rd; ($user, $pass, $uid, $gid, $info, $home, $shell) = split(/[:]/, $__rd, 7); 1 } else { 0 } }) {
        if (int($uid) > int(1000) && (!((($__nocasematch) ? lc($shell) eq lc('/bin/false') : $shell eq '/bin/false')))) {
            if (($home =~ '^/home/')) {
                say "User: $user (UID: $uid) - $home";
                $? = 0 << 8; $CHILD_ERROR = $? >> 8;
            } else {
                            }
        }
    }
if ($__sh2_is0) { open(STDIN, '<&', $__sh2_is0); close($__sh2_is0); }
}
@numbers = ("1", "2", "3", "4", "5", "6", "7", "8", "9", "10");  # line 115
@middle = (join(' ', __sh_slice(\@numbers, 3, 4)));  # line 116
@first_half = (join(' ', __sh_slice(\@numbers, 0, 5)));  # line 117
@last_half = (join(' ', @numbers[5..$#numbers]));  # line 118
for my $__file ((__sh_glob('*.txt'), __sh_glob('*.log'), __sh_glob('*.dat'))) {
    $file = $__file;
    if (($file) =~ /^(?:.*\.txt|.*\.log)$/) {
        say "Text file: $file";
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
            } elsif (($file) =~ /^(?:.*\.dat)$/) {
        say "Data file: $file";
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
            } else {
        say "Other file: $file";
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
            }
}
{  # line 130
    local %ENV = %ENV;
    $ENV{DEBUG} = "1";
        $ENV{LOG_LEVEL} = "verbose";
        {
        local %ENV = %ENV;
        do { { no strict 'refs'; undef ${'DEBUG'}; undef @{'DEBUG'}; undef %{'DEBUG'}; delete $ENV{'DEBUG'}; }
$? = 0 << 8; $CHILD_ERROR = $? >> 8; };
        print("Inner: LOG_LEVEL=" . ($ENV{LOG_LEVEL}) . ", DEBUG=" . (((defined($ENV{DEBUG}) && length($ENV{DEBUG})) ? $ENV{DEBUG} : "unset")), "\n");
            }
    print("Outer: LOG_LEVEL=" . ($ENV{LOG_LEVEL}) . ", DEBUG=" . ($ENV{DEBUG}), "\n");
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    }
$message = "Hello " . (((defined($ENV{USER}) && length($ENV{USER})) ? $ENV{USER} : "guest")) . " from " . (((defined($ENV{HOSTNAME}) && length($ENV{HOSTNAME})) ? $ENV{HOSTNAME} : "localhost"));  # line 143
say $message;  # line 144
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;END {  # line 160
print 'Cleaning up...', "\n";
unlink glob('/tmp/064_hard_temp_*');
}
$input = ((defined($__sh_args[0]) && length($__sh_args[0])) ? $__sh_args[0] : "default_value");  # line 163
$sanitized = (($input =~ s/[^a-zA-Z0-9]/_/gr));  # line 164
$uppercase = uc($sanitized);  # line 165
print "Input: '" . ($input) . "' -> Sanitized: '" . ($sanitized) . "' -> Uppercase: '" . ($uppercase) . "'\n";  # line 166
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$CHILD_ERROR = (-f $__sh_args[0]) ? 0 : 1;
if (-f $__sh_args[0]) {
    print "File exists\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
} else {
    print "File not found\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
do { my $__sh_ok = (-d $__sh_args[1]); $? = ($__sh_ok ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; if ($__sh_ok) { $__sh_ok = chdir($__sh_args[1]) ? 1 : 0; $? = ($__sh_ok ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; } if ($__sh_ok) { my $__sh_cap; do { $__sh_cap = (require Cwd, Cwd::getcwd()); $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; print $__sh_cap, "\n" if defined $__sh_cap; $__sh_ok = ($CHILD_ERROR == 0); } if (!$__sh_ok) { print "Directory not accessible\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_ok = ($CHILD_ERROR == 0); } };
print "Script completed successfully!\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
