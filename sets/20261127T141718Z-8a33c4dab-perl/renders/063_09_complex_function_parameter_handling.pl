#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $__argc = @ARGV;
my $key;
my $value;
my %options;
my $i;
my $flags;
my $j;
my @args;
my %args;
my $options;
my $args = '';

sub complex_function {
    local @ARGV = @_;
    my $__argc = @ARGV;
    my @args = (@ARGV);
        my %options = ();
        my $i = "0";
    my $__sh_loop_rc0 = 0;
    while (($i < scalar(@args))) {
        $? = 0 << 8;
        if (($args[$i]) =~ /^(?:--.*)$/) {
            my $key = (($args[$i] =~ s{^--}{}r));
                        my $value = ((defined($args[$i+1]) && length($args[$i+1])) ? $args[$i+1] : "true");
                        $options{$key} = $value;
            $i = int($i+=2);
        } elsif (($args[$i]) =~ /^(?:-.*)$/) {
            my $flags = (($args[$i] =~ s{^-}{}r));
                        my $j = "0";
            while (($j < length($flags))) {
                $options{substr($flags, $j, 1)} = "true";
                (($j++));
            }
            $? = (((($i++)) != 0) ? 0 : 1) << 8;
        } else {
            $__sh_loop_rc0 = 0; last;
        }
        $__sh_loop_rc0 = $? >> 8;
    }
$? = ($__sh_loop_rc0) << 8;
    return "Processed " . (join(' ', scalar(keys %options))) . " options";
}
say complex_function("--flag1", "--option1=value1", "-abc");
$? = 0 << 8;

exit $? >> 8;
