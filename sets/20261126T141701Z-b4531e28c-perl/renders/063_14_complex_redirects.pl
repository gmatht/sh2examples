#!/usr/bin/env perl
use strict;
use warnings;
use English qw(-no_match_vars $ERRNO $EVAL_ERROR $INPUT_RECORD_SEPARATOR $OS_ERROR $PROGRAM_NAME);

our $CHILD_ERROR = 0;
use feature 'say';

my $__ps_tmp0;
my $__ps_tmp1;
my $__ps_status0;
my $a = '';
my $b = '';

$__ps_tmp0 = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
do { my @__sl = do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__fh, '<', 'file1.txt') or do { $CHILD_ERROR = 2; () }; my $__c = <$__fh> // ''; close $__fh; split(/\n/, $__c) }; pop @__sl if @__sl && $__sl[-1] eq q{}; @__sl = sort { our ($a, $b); $a cmp $b } @__sl; open(my $__ofh, '>', $__ps_tmp0) or die "sort: $__ps_tmp0: $ERRNO"; if (@__sl) { print $__ofh join("\n", @__sl), "\n"; } close $__ofh; }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;$__ps_tmp1 = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
do { my @__sl = do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__fh, '<', 'file2.txt') or do { $CHILD_ERROR = 2; () }; my $__c = <$__fh> // ''; close $__fh; split(/\n/, $__c) }; pop @__sl if @__sl && $__sl[-1] eq q{}; @__sl = sort { our ($a, $b); $a cmp $b } @__sl; open(my $__ofh, '>', $__ps_tmp1) or die "sort: $__ps_tmp1: $ERRNO"; if (@__sl) { print $__ofh join("\n", @__sl), "\n"; } close $__ofh; }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;open(my $__sh2_rfh1, '>&', \*STDOUT) or die "Cannot dup stdout: $!\n";
close(STDOUT); open(STDOUT, '>', "comparison.txt") or die "Cannot write to comparison.txt: $!\n";
my $__sh2_rsel1 = select(STDOUT);
my $__rc = 0; do { my @__a = split(/\n/, do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__f, '<', $__ps_tmp0) or die "diff: $__ps_tmp0: $ERRNO"; my $__c = <$__f>; close $__f; $__c }, -1); pop @__a if @__a && $__a[-1] eq q{}; my @__b = split(/\n/, do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__g, '<', $__ps_tmp1) or die "diff: $__ps_tmp1: $ERRNO"; my $__c = <$__g>; close $__g; $__c }, -1); pop @__b if @__b && $__b[-1] eq q{}; my @__dp; for my $__i (0..$#__a) { for my $__j (0..$#__b) { if ($__a[$__i] eq $__b[$__j]) { $__dp[$__i+1][$__j+1] = $__dp[$__i][$__j] + 1; } else { $__dp[$__i+1][$__j+1] = ($__dp[$__i][$__j+1] > $__dp[$__i+1][$__j]) ? $__dp[$__i][$__j+1] : $__dp[$__i+1][$__j]; } } } my ($__i, $__j) = ($#__a, $#__b); my @__ops; while ($__i >= 0 && $__j >= 0) { if ($__a[$__i] eq $__b[$__j]) { unshift @__ops, [0, $__i, $__j]; $__i--; $__j--; } elsif ($__dp[$__i][$__j+1] >= $__dp[$__i+1][$__j]) { unshift @__ops, [-1, $__i, $__j]; $__i--; } else { unshift @__ops, [1, $__i, $__j]; $__j--; } } while ($__i >= 0) { unshift @__ops, [-1, $__i, -1]; $__i--; } while ($__j >= 0) { unshift @__ops, [1, -1, $__j]; $__j--; }  my $__is_out = 0; my $__fh = \*STDOUT; my $__changed = 0; my $__idx = 0; my $__common = -1; while ($__idx < @__ops) { if ($__ops[$__idx][0] == 0) { $__common = $__ops[$__idx][1] + 1; $__idx++; next; } my @__dels; my @__adds; my $__last_a = -1; my $__last_b = -1; while ($__idx < @__ops && $__ops[$__idx][0] != 0) { if ($__ops[$__idx][0] == -1) { push @__dels, $__ops[$__idx][1]; $__last_a = $__ops[$__idx][1]; } else { push @__adds, $__ops[$__idx][2]; $__last_b = $__ops[$__idx][2]; } $__idx++; } $__changed = 1; my $__a1 = $__dels[0] + 1; my $__a2 = $__dels[-1] + 1; my $__b1 = $__adds[0] + 1; my $__b2 = $__adds[-1] + 1; my $__astr = $__a1 == $__a2 ? "$__a1" : "$__a1,$__a2"; my $__bstr = $__b1 == $__b2 ? "$__b1" : "$__b1,$__b2"; my $__opch = (@__dels && @__adds) ? "c" : (@__dels ? "d" : "a"); my $__hdr; if ($__opch eq "c") { $__hdr = "$__astr$__opch$__bstr"; } elsif ($__opch eq "d") { $__hdr = "$__astr$__opch" . (defined $__last_b ? $__last_b + 1 : 1); } else { my $__apos = $__common; $__hdr = "$__apos$__opch$__bstr"; } print $__fh "$__hdr\n"; for my $__k (@__dels) { print $__fh "< $__a[$__k]\n"; } if (@__dels && @__adds) { print $__fh "---\n"; } for my $__k (@__adds) { print $__fh "> $__b[$__k]\n"; } } $__rc = $__changed ? 1 : 0; close $__fh if $__is_out; }; $? = ($__rc) << 8; $CHILD_ERROR = $? >> 8; select($__sh2_rsel1); open(STDOUT, '>&', $__sh2_rfh1) or die "Cannot restore stdout: $!\n"; close($__sh2_rfh1);
$__ps_status0 = $CHILD_ERROR;
do { unlink($__ps_tmp0, $__ps_tmp1); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
$CHILD_ERROR = $__ps_status0;
say "exit: $CHILD_ERROR";  # line 6
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
