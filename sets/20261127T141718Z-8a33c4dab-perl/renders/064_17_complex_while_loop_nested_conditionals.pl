#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;
use feature 'say';

my $user = '';
my $pass = '';
my $uid = '';
my $gid = '';
my $info = '';
my $home = '';
my $shell = '';

{
    open(my $__sh2_is0, '<&', \*STDIN);
    open(STDIN, '<', "/etc/passwd") or die "Cannot read input: $!\n";
    my $__sh_loop_rc0 = 0;
    while (do { my $__rd = <STDIN>; if (defined $__rd) { chomp $__rd; ($user, $pass, $uid, $gid, $info, $home, $shell) = split(/[:]/, $__rd, 7); 1 } else { 0 } }) {
        if (int($uid) > int(1000) && $shell ne "/bin/false") {
            $CHILD_ERROR = (($home =~ '^/home/')) ? 0 : 1;
            if (($home =~ '^/home/')) {
                say "User: $user (UID: $uid) - $home";
                $? = 0 << 8; $CHILD_ERROR = $? >> 8;
            } else {
                $CHILD_ERROR = 0;
            }
        }
        $__sh_loop_rc0 = $? >> 8;
    }
$? = ($__sh_loop_rc0) << 8; $CHILD_ERROR = $? >> 8;
if ($__sh2_is0) { open(STDIN, '<&', $__sh2_is0); close($__sh2_is0); }
}

exit $? >> 8;
