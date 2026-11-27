#!/usr/bin/env perl
use strict;
our @__sh_args;
our $__sh_args_n;

use warnings;
local @__sh_args = @ARGV;
local $__sh_args_n = scalar(@ARGV);
use feature 'say';

our $CHILD_ERROR = 0;
my $File;

my $__argc = @__sh_args;
my @numbers;
my @letters;
my $num;
my $letter;
my $data_type;
my $value;
my $file_count;
my $dir_count;
my $user;
my $host;
my $port;
my %matrix;
my $i;
my $j;
my $a;
my $b;
my $c;
my $result;
my $file_info;
my @matrix;
my $test_string = '';
my $matrix;

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

print "=== Advanced Bash Idioms Examples ===\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "\n";  # line 7
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "1. Nested loops with conditional logic and array manipulation:\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
@numbers = ("1", "2", "3", "4", "5");  # line 11
@letters = ("a", "b", "c", "d", "e");  # line 12
for my $__num ((@numbers)) {
    $num = $__num;
    for my $__letter ((@letters)) {
        $letter = $__letter;
        if (int($num) > int(3) && $letter ne "c") {
            print "  Number " . ($num) . " with letter " . ($letter) . " (filtered)\n";
            $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        }
    }
}
print "\n";  # line 20
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "2. Function with nested case statements and parameter expansion:\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
sub process_data {
    local @__sh_args = @_;
    my $__argc = @__sh_args;
    my $data_type = $__sh_args[0];
        my $value = $__sh_args[1];
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    if (($data_type) =~ /^(?:string)$/) {
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        if ((lc($value)) =~ /^(?:hello|hi)$/) {
            return "  Greeting detected: " . ($value);
        } elsif ((lc($value)) =~ /^(?:bye|goodbye)$/) {
            return "  Farewell detected: " . ($value);
        } else {
            return "  Unknown string: " . ($value);
        }
    } elsif (($data_type) =~ /^(?:number)$/) {
        $CHILD_ERROR = (($value =~ '^[0-9]+$')) ? 0 : 1;
        if (($value =~ '^[0-9]+$')) {
            if (($value % 2 == 0)) {
                return "  Even number: " . ($value);
            } else {
                return "  Odd number: " . ($value);
            }
        } else {
            return "  Invalid number: " . ($value);
        }
    } else {
        return "  Unknown data type: " . ($data_type);
    }
}
say process_data("string", "Hello");
say process_data("string", "Bye");
say process_data("number", "42");
say process_data("number", "17");
print "\n";  # line 63
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "3. Complex conditional with command substitution and arithmetic:\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$file_count = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); do { my @__pl = split(/\n/, do { require File::Find; my @__r; my $__max = 1; File::Find::find({ no_chdir => 1, wanted => sub { my $__p = $File::Find::name; my $__dep = ($__p =~ tr{/}{/}); if ($__max >= 0 && $__dep >= $__max) { $File::Find::prune = 1; } return if $__max >= 0 && $__dep > $__max; return unless -f $__p && !-l $__p;  push @__r, $__p; } }, '.'); join("\n", @__r) }); print scalar(@__pl), "\n"; $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };  # line 67
$dir_count = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); do { my @__pl = split(/\n/, do { require File::Find; my @__r; my $__max = 1; File::Find::find({ no_chdir => 1, wanted => sub { my $__p = $File::Find::name; my $__dep = ($__p =~ tr{/}{/}); if ($__max >= 0 && $__dep >= $__max) { $File::Find::prune = 1; } return if $__max >= 0 && $__dep > $__max; return unless -d $__p && !-l $__p;  push @__r, $__p; } }, '.'); join("\n", @__r) }); print scalar(@__pl), "\n"; $? = 0 << 8; $CHILD_ERROR = $? >> 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };  # line 68
if (int($file_count) > int(0) && int($dir_count) > int(1)) {
    if (($file_count > $dir_count)) {
        print "  More files (" . ($file_count) . ") than directories (" . ($dir_count) . ")\n";
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    } else {
        if (($file_count == $dir_count)) {
            print "  Equal count: " . ($file_count) . " files and " . ($dir_count) . " directories\n";
            $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        } else {
            print "  More directories (" . ($dir_count) . ") than files (" . ($file_count) . ")\n";
            $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        }
    }
} else {
    print "  Insufficient items for comparison\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
print "\n";  # line 81
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "4. Nested here-documents with parameter expansion:\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$user = "admin";
$host = "localhost";
$port = 22;
print('    SSH Configuration:
    $(cat <<\'INNER\'
        User: $user
        Host: $host
        Port: $port
        Status: $(ping -c 1 $host >/dev/null 2>&1 && echo "Online" || echo "Offline")
INNER
    )
');
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "\n";  # line 99
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "5. Array processing with nested loops and conditional logic:\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
%matrix = ();  # line 103
$matrix{'0,0'} = "1";  # line 104
$matrix{'0,1'} = "2";  # line 104
$matrix{'0,2'} = "3";  # line 104
$matrix{'1,0'} = "4";  # line 105
$matrix{'1,1'} = "5";  # line 105
$matrix{'1,2'} = "6";  # line 105
$matrix{'2,0'} = "7";  # line 106
$matrix{'2,1'} = "8";  # line 106
$matrix{'2,2'} = "9";  # line 106
my $__sh_loop_rc0 = 0;
for my $__i (("0", "1", "2")) {
    $i = $__i;
    my $__sh_loop_rc1 = 0;
    for my $__j (("0", "1", "2")) {
        $j = $__j;
        $value = ${matrix}{"$i,$j"};
        if (int($value) > int(5)) {
            print "  [$value] ";
            $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        } else {
            print "  $value ";
            $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        }
        $__sh_loop_rc1 = $? >> 8;
    }
$? = ($__sh_loop_rc1) << 8; $CHILD_ERROR = $? >> 8;
    print "\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    $__sh_loop_rc0 = $? >> 8;
}
$? = ($__sh_loop_rc0) << 8; $CHILD_ERROR = $? >> 8;
print "\n";  # line 119
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "6. Process substitution with nested commands and error handling:\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print("  First word: " . ((($test_string =~ s{ .*$}{}r))), "\n");
print("  Last word: " . ((($test_string =~ s{^.* }{}r))), "\n");
print("  Middle: " . ((($test_string =~ s{^.*? }{}r))), "\n");
print("  Middle: " . ((($test_string =~ s{ [^ ]*$}{}r))), "\n");
print("  Uppercase: " . (uc($test_string)), "\n");
print("  Lowercase: " . (lc($test_string)), "\n");
print("  Capitalize: " . (ucfirst($test_string)), "\n");
print "\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "11. Complex arithmetic with nested expressions:\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$a = "10";
$b = "5";
$c = "3";
$result = int(((($a + $b) * $c) - (do { my $__d = $c; eval { int(((do { my $__d = $b; eval { int(($a)) % ($__d) } })) / ($__d)) } })));
print "  Expression: (a + b) * c - (a % b) / c\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "  Values: a=$a, b=$b, c=$c";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "  Result: $result";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
if ((($a > $b) && ($b < $c) || ($a % 2 == 0))) {
    print "  Complex condition met: a > b AND (b < c OR a is even)\n";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
print "\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "12. Nested command substitution with error handling:\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print("  Current directory: " . (do { my $__sh_cap; $__sh_cap = (require Cwd, Cwd::getcwd()); $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap }), "\n");
print("  Parent directory: " . (do { my $__sh_cap; $__sh_cap = (do { require File::Basename; File::Basename::dirname(do { my $__sh_cap; $__sh_cap = (require Cwd, Cwd::getcwd()); $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap }) }); $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap }), "\n");
print("  Home directory: " . (do { my $__sh_cap; $__sh_cap = (do { require File::Basename; File::Basename::dirname(do { my $__sh_cap; $__sh_cap = (do { require File::Basename; File::Basename::dirname(do { my $__sh_cap; $__sh_cap = (require Cwd, Cwd::getcwd()); $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap }) }); $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap }) }); $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap }), "\n");
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$file_info = do { my $__c = __sh_pipe_capture(q{'stat' '-c' '%s %y' 'nonexistent_file' 2>/dev/null}); if ($CHILD_ERROR != 0) { $__c = __sh_pipe_capture(q{'echo' 'File not found'}); } $__c };
say "  File info: $file_info";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "=== Advanced Bash Idioms Examples Complete ===\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
