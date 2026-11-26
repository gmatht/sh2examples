#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;
my $d;

$d = do { my $__sh_cap; $__sh_cap = do { require File::Temp; File::Temp::tempdir('sh2_ps_XXXXXX', TMPDIR => 1, CLEANUP => 0) }; $? = 0 << 8; $CHILD_ERROR = $? >> 8; $__sh_cap =~ s/\n+\z// if defined $__sh_cap; $__sh_cap };  # line 3
do { require File::Path; File::Path::make_path(($d) . "/a"); my $__ok = 1; $? = ($__ok ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; };  # line 4
chdir(($d) . "/a") or do { exit "1"; };  # line 5
chdir("..") or die "cd: $!\n";$? = 0 << 8; $CHILD_ERROR = $? >> 8;  # line 6
print("parent contains a: " . (do { my $__cap = ''; { local *STDOUT; open(STDOUT, '>', \$__cap) or die "capture: $!\n";     $? = ((-d 'a') ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;  # line 7
    if ($CHILD_ERROR == 0) {
        print "yes\n";
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    }
    if ($CHILD_ERROR != 0) {
        print "no\n";
        $? = 0 << 8; $CHILD_ERROR = $? >> 8;
    }
;; } $__cap =~ s/\n+\z//; $__cap }) . "\n");
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
chdir("/") or die "cd: $!\n";  # line 8
do { require File::Path; File::Path::remove_tree($d); $? = 0 << 8; $CHILD_ERROR = $? >> 8; };  # line 9

exit $? >> 8;
