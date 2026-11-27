#!/usr/bin/env perl
use strict;
use warnings;
our $CHILD_ERROR = 0;
use feature 'say';

my $__argc = @ARGV;
my $file;
my $count;
my @files;

sub process_files {
    local @ARGV = @_;
    my $__argc = @ARGV;
    my @files = (@ARGV);
        my $count = "0";
    my $__sh_loop_rc0 = 0;
    for my $__file ((@files)) {
        $file = $__file;
        $CHILD_ERROR = (-f $file) ? 0 : 1;
        if (-f $file) {
            $? = (((($count++)) != 0) ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8;
            say "Processing: $file";
            $? = 0 << 8; $CHILD_ERROR = $? >> 8;
        } else {
            $CHILD_ERROR = 0;
        }
        $__sh_loop_rc0 = $? >> 8;
    }
$? = ($__sh_loop_rc0) << 8; $CHILD_ERROR = $? >> 8;
    say "Total files processed: $count";
    $? = 0 << 8; $CHILD_ERROR = $? >> 8;
}
process_files("file1.txt", "file2.txt", "nonexistent.txt");

exit $? >> 8;
