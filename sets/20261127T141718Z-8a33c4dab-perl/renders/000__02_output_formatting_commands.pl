#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';
use English qw(-no_match_vars $ERRNO $EVAL_ERROR $INPUT_RECORD_SEPARATOR $OS_ERROR $PROGRAM_NAME);

my $d;
my $sha256_result;
my $sha512_result;
my $strings_result;
my $tee_result;
my $perl_result;

print "=== Output and Formatting Commands ===\n";
$? = 0 << 8;
say "Echo result: Hello from backticks";
$? = 0 << 8;
say "Printf result: Number: 42, String: test";
$? = 0 << 8;
print "=== Compression Commands ===\n";
$? = 0 << 8;
print "=== Network Commands ===\n";
$? = 0 << 8;
print "=== Process Management Commands ===\n";
$? = 0 << 8;
$d = do { my $__sh_cap; $__sh_cap = do { require File::Temp; File::Temp::tempdir('sh2_ps_XXXXXX', TMPDIR => 1, CLEANUP => 0) }; $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 56
chdir($d) or do { warn "cd: " . $d . ": $!\n"; exit "1"; };  # line 57
print "=== Checksum Commands ===\n";
$? = 0 << 8;
open my $__fh, '>', "test_checksum.txt" or die "Cannot write file: $!\n";
print {$__fh} "test content\n";
close $__fh;
$? = 0 << 8;
$sha256_result = do { my $__sh_cap; $__sh_cap = do { require Digest::SHA; if (open(my $__fh, '<:raw', 'test_checksum.txt')) { my $__h = Digest::SHA->new(256); $__h->addfile($__fh); close $__fh; $__h->hexdigest() . q{  } . 'test_checksum.txt' } else { print STDERR 'sha256sum' . q{: } . 'test_checksum.txt' . q{: } . $ERRNO . qq{\n}; $? = 1 << 8; '' } }; $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 67
say "SHA256 result: $sha256_result";  # line 68
$? = 0 << 8;
$sha512_result = do { my $__sh_cap; $__sh_cap = do { require Digest::SHA; if (open(my $__fh, '<:raw', 'test_checksum.txt')) { my $__h = Digest::SHA->new(512); $__h->addfile($__fh); close $__fh; $__h->hexdigest() . q{  } . 'test_checksum.txt' } else { print STDERR 'sha512sum' . q{: } . 'test_checksum.txt' . q{: } . $ERRNO . qq{\n}; $? = 1 << 8; '' } }; $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 72
say "SHA512 result: $sha512_result";  # line 73
$? = 0 << 8;
$strings_result = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); do { my @__pl = split(/\n/, do { local $/; if (open(my $__f, '<', 'test_binary.txt')) { my $__c = <$__f>; close $__f; join("\n", ($__c =~ /([\x20-\x7e]{4,})/g)) } else { '' } }); pop @__pl if @__pl && $__pl[-1] eq ''; @__pl = @__pl[0..2] if @__pl > 3; say join("\n", @__pl) if @__pl; $? = 0 << 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };  # line 77
print "Strings result:\n";
$? = 0 << 8;
say $strings_result;  # line 79
$? = 0 << 8;
print "=== I/O Redirection Commands ===\n";
$? = 0 << 8;
$tee_result = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); do { open(my $__t, '>', 'test_tee.txt') or die "tee: 'test_tee.txt': $!\n"; print {$__t} 'test output', "\n"; close($__t); print 'test output', "\n"; $? = 0 << 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };
say "Tee result: $tee_result";  # line 86
$? = 0 << 8;
print "=== Perl Command ===\n";
$? = 0 << 8;
$perl_result = do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     { no strict 'vars'; no warnings; print "Hello from Perl\n"; }  # line 92
    $? = 0 << 8;
;; } $__cap =~ s/\n+\z//; $__cap };
say "Perl result: $perl_result";  # line 93
$? = 0 << 8;
do { unlink('test_checksum.txt', 'test_tee.txt'); $? = 0 << 8; };  # line 96
if (chdir("/")) {  # line 97
    
} else {
    warn "cd: " . "/" . ": $!\n";
    
}
do { require File::Path; File::Path::remove_tree($d); $? = 0 << 8; };  # line 98

exit $? >> 8;
