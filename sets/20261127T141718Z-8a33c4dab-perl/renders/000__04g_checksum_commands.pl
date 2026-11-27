#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';
use English qw(-no_match_vars $ERRNO $EVAL_ERROR $INPUT_RECORD_SEPARATOR $OS_ERROR $PROGRAM_NAME);

my $d;
my $sha256_result;
my $sha512_result;
my $strings_result;

print "=== Checksum Commands ===\n";
$? = 0 << 8;
$d = do { my $__sh_cap; $__sh_cap = do { require File::Temp; File::Temp::tempdir('sh2_ps_XXXXXX', TMPDIR => 1, CLEANUP => 0) }; $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 10
chdir($d) or do { warn "cd: " . $d . ": $!\n"; exit "1"; };  # line 11
open my $__fh, '>', "test_checksum.txt" or die "Cannot write file: $!\n";
print {$__fh} "test content\n";
close $__fh;
$? = 0 << 8;
$sha256_result = do { my $__sh_cap; $__sh_cap = do { require Digest::SHA; if (open(my $__fh, '<:raw', 'test_checksum.txt')) { my $__h = Digest::SHA->new(256); $__h->addfile($__fh); close $__fh; $__h->hexdigest() . q{  } . 'test_checksum.txt' } else { print STDERR 'sha256sum' . q{: } . 'test_checksum.txt' . q{: } . $ERRNO . qq{\n}; $? = 1 << 8; '' } }; $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 13
say "SHA256 result: $sha256_result";  # line 14
$? = 0 << 8;
$sha512_result = do { my $__sh_cap; $__sh_cap = do { require Digest::SHA; if (open(my $__fh, '<:raw', 'test_checksum.txt')) { my $__h = Digest::SHA->new(512); $__h->addfile($__fh); close $__fh; $__h->hexdigest() . q{  } . 'test_checksum.txt' } else { print STDERR 'sha512sum' . q{: } . 'test_checksum.txt' . q{: } . $ERRNO . qq{\n}; $? = 1 << 8; '' } }; $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 18
say "SHA512 result: $sha512_result";  # line 19
$? = 0 << 8;
$strings_result = do { my $__cap = ''; open my $__fh, '>', \$__cap or die; my $__old = select($__fh); do { my @__pl = split(/\n/, do { local $/; if (open(my $__f, '<', 'target/debug/debashc.exe')) { my $__c = <$__f>; close $__f; join("\n", ($__c =~ /([\x20-\x7e]{4,})/g)) } else { '' } }); pop @__pl if @__pl && $__pl[-1] eq ''; @__pl = @__pl[0..2] if @__pl > 3; say join("\n", @__pl) if @__pl; $? = 0 << 8; }; close $__fh; select($__old); $__cap =~ s/\n+\z//; $__cap };  # line 23
print "Strings result:\n";
$? = 0 << 8;
say $strings_result;  # line 25
$? = 0 << 8;
do { unlink('test_checksum.txt'); $? = 0 << 8; };  # line 28
print "=== Checksum Commands Complete ===\n";
$? = 0 << 8;
if (chdir("/")) {  # line 31
    
} else {
    warn "cd: " . "/" . ": $!\n";
    
}
do { require File::Path; File::Path::remove_tree($d); $? = 0 << 8; };  # line 32

exit $? >> 8;
