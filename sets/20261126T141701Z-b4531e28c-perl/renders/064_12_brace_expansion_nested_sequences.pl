#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

our $CHILD_ERROR = 0;

do { require File::Path; File::Path::make_path('project/src/main/java', 'project/src/main/resources', 'project/src/test/java', 'project/src/test/resources', 'project/docs/api', 'project/docs/user', 'project/build/classes', 'project/build/lib'); my $__ok = 1; $? = ($__ok ? 0 : 1) << 8; $CHILD_ERROR = $? >> 8; };
print "Project structure created\n";
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
say "done: $CHILD_ERROR";  # line 7
$? = 0 << 8; $CHILD_ERROR = $? >> 8;
$? = 0 << 8; $CHILD_ERROR = $? >> 8;

exit $? >> 8;
