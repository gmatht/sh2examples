#!/usr/bin/env perl
use strict;
use warnings;
my $d;

$d = do { my $__sh_cap; $__sh_cap = do { require File::Temp; File::Temp::tempdir('sh2_ps_XXXXXX', TMPDIR => 1, CLEANUP => 0) }; $? = 0 << 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 3
do { my $__ok = 1; for my $__d (($d) . "/a") { my $__acc = $__d =~ m{^/} ? '' : '.'; for my $__c (split m{/+}, $__d) { next if $__c eq '' || $__c eq '.'; $__acc .= "/$__c"; next if -d $__acc; unless (mkdir($__acc)) { $__ok = 0; print STDERR "mkdir: $!\n"; last; } } } $? = ($__ok ? 0 : 1) << 8; };  # line 4
chdir(($d) . "/a") or do { warn "cd: " . ($d) . "/a" . ": $!\n"; exit "1"; };  # line 5
if (chdir("..")) {  # line 6
    $? = 0 << 8;
} else {
    warn "cd: " . ".." . ": $!\n";
    $? = 1 << 8;
}
print("parent contains a: " . (do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     $? = ((-d 'a') ? 0 : 1) << 8;  # line 7
    if (($? >> 8) == 0) {
        print "yes\n";
        $? = 0 << 8;
    }
    if (($? >> 8) != 0) {
        print "no\n";
        $? = 0 << 8;
    }
;; } $__cap =~ s/\n+\z//; $__cap }) . "\n");
$? = 0 << 8;
if (chdir("/")) {  # line 8
    
} else {
    warn "cd: " . "/" . ": $!\n";
    
}
do { require File::Path; File::Path::remove_tree($d); $? = 0 << 8; };  # line 9

exit $? >> 8;
