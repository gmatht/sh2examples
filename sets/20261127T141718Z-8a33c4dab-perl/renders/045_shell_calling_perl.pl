#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;
my $arg;

my $SHELL_VAR;

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

print "Warmup 1\n";  # line 3
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my @__l = split(/\n/, 'apple', -1); pop @__l if @__l && $__l[-1] eq q{}; for my $__l (@__l) { local $_ = $__l . "\n"; print "Fruit: $_\n" } }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;print "=== Example 1: Simple Perl one-liner ===\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
{ no strict 'vars'; no warnings; print "Hello from Perl!\n"; }  # line 12
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "\n=== Example 2: Perl with arguments ===\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
{ no strict 'vars'; no warnings; @ARGV = ("first", "second", "third"); }
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
{ no strict 'vars'; no warnings; foreach $arg (@ARGV) { print "Argument: $arg\n" } }
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
print "\n=== Example 3: Perl processing shell variables ===\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$SHELL_VAR = "Hello World";
$ENV{SHELL_VAR} = ${SHELL_VAR} if exists $ENV{SHELL_VAR};
__sh_uu_run('perl', '-e', 'print "Shell variable: $ENV{SHELL_VAR}\\n"');  # line 21
$ENV{SHELL_VAR} = ${SHELL_VAR};  # line 22
__sh_uu_run('perl', '-e', 'print "Shell variable: $ENV{SHELL_VAR}\\n"');  # line 23
print "\n=== Example 4: Perl reading from pipeline ===\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my @__l = split(/\n/, q{apple
banana
cherry}, -1); pop @__l if @__l && $__l[-1] eq q{}; for my $__l (@__l) { local $_ = $__l . "\n"; chomp; print "Fruit: $_\n" } }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;print "\n=== Example 5: Perl script with here document ===\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
system('bash', '-c', q['perl' <<'_SH2DOC_'
use strict;
use warnings;

my @numbers = (1, 2, 3, 4, 5);
my $sum = 0;

foreach my $num (@numbers) {
    $sum += $num;
    print "Added $num, sum is now $sum\n";
}

print "Final sum: $sum\n";
_SH2DOC_]);$CHILD_ERROR = $? >> 8;  # line 31

exit $? >> 8;
