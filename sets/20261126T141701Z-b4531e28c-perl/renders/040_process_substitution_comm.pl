#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';
use English qw(-no_match_vars $ERRNO $EVAL_ERROR $INPUT_RECORD_SEPARATOR $OS_ERROR $PROGRAM_NAME);

our $CHILD_ERROR = 0;
my $__ps_tmp0;
my $__ps_tmp1;
my $__ps_status0;

$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 4
print "== Process substitution with comm ==\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$__ps_tmp0 = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
open my $__fh, '>', $__ps_tmp0 or die "Cannot write file: $!\n";
print {$__fh} "a\nb\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$__ps_tmp1 = do { my $__sh_cap; $__sh_cap = do { require File::Temp; (File::Temp::tempfile('sh2_ps_XXXXXX', TMPDIR => 1, UNLINK => 0))[1] }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };
open my $__fh, '>', $__ps_tmp1 or die "Cannot write file: $!\n";
print {$__fh} "b\nc\n";
close $__fh;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
do { my @__a = split(/\n/, do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__f, '<', $__ps_tmp0) or die "comm: $__ps_tmp0: $ERRNO"; my $__c = <$__f>; close $__f; $__c }, -1); pop @__a if @__a && $__a[-1] eq q{}; my @__b = split(/\n/, do { local $INPUT_RECORD_SEPARATOR = undef; open(my $__g, '<', $__ps_tmp1) or die "comm: $__ps_tmp1: $ERRNO"; my $__c = <$__g>; close $__g; $__c }, -1); pop @__b if @__b && $__b[-1] eq q{}; my ($__i, $__j) = (0, 0); my @__out; while ($__i < @__a && $__j < @__b) { my $__c = $__a[$__i] cmp $__b[$__j]; if ($__c == 0) { push @__out, $__a[$__i]; $__i++; $__j++; } elsif ($__c < 0) { $__i++; } else { $__j++; } } if (@__out) { print STDOUT join("\n", @__out), "\n"; } }; $? = 0 << 8; $CHILD_ERROR = $? >> 8;$__ps_status0 = $CHILD_ERROR;
do { unlink($__ps_tmp0, $__ps_tmp1); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };
$CHILD_ERROR = $__ps_status0;
say "exit: $CHILD_ERROR";  # line 9
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
