#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';
use Sys::Hostname qw(hostname);
$ENV{HOSTNAME} //= hostname();

my $message;

$message = "Hello " . (((defined($ENV{USER}) && length($ENV{USER})) ? $ENV{USER} : "guest")) . " from " . (((defined($ENV{HOSTNAME}) && length($ENV{HOSTNAME})) ? $ENV{HOSTNAME} : "localhost"));  # line 4
say $message;  # line 5
$? = 0 << 8;

exit $? >> 8;
