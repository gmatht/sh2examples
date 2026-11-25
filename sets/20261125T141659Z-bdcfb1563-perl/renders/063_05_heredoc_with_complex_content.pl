#!/usr/bin/env perl
use strict;
use warnings;
my $var = '';

system('bash', '-c', q['cat' <<'_SH2DOC_' | 'grep' '-v' '^#' | 'sed' 's/^/  /'
# This is a comment
$(echo "Command substitution")
${var:-default}
$(( 1 + 2 * 3 ))
_SH2DOC_]);

exit $? >> 8;
