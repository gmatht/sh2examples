#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my $hello;

my $__argc = @ARGV;
my $n;
my $rovar;
my $lc;
my $uc;
my $myexport;
my @arr;
my $original;
my $global_var;
my $tracetest;
my $printtest;
my $comb;
my $comb2;
my @singlearr;
my $plain;
my %assoc;
my $x = '';
my $arr;
my $singlearr;
my $assoc = '';

print "=== typeset -i (integer attribute) ===\n";
$? = 0 << 8;
do { { no strict 'refs'; undef ${'n'}; undef @{'n'}; undef %{'n'}; delete $ENV{'n'}; }  # line 10
$? = 0 << 8; };
do { $n = '42';  # line 11
$? = 0 << 8; };
print "After 'typeset -i n=42': n='" . ($n) . "'\n";  # line 12
$? = 0 << 8;
$n = int(($n + 1));
print "After 'n=n+1':          n='" . ($n) . "' (integer arithmetic applied)\n";  # line 14
$? = 0 << 8;
$n = int($hello);
print "After 'n=\"hello\"':     n='" . ($n) . "' (assigns 0; non-numeric string becomes 0)\n";  # line 16
$? = 0 << 8;
print "\n";  # line 17
$? = 0 << 8;
print "=== typeset -r (readonly attribute) ===\n";
$? = 0 << 8;
do { { no strict 'refs'; undef ${'rovar'}; undef @{'rovar'}; undef %{'rovar'}; delete $ENV{'rovar'}; }  # line 21
$? = 0 << 8; };
$rovar = "immutable";  # line 22
print "After 'typeset -r rovar=immutable': rovar='" . ($rovar) . "'\n";  # line 23
$? = 0 << 8;
print "(Attempting 'rovar=change' would cause an error; skipped for safety.)\n";
$? = 0 << 8;
print "\n";  # line 25
$? = 0 << 8;
print "=== typeset -l (lowercase attribute) ===\n";
$? = 0 << 8;
do { { no strict 'refs'; undef ${'lc'}; undef @{'lc'}; undef %{'lc'}; delete $ENV{'lc'}; }  # line 29
$? = 0 << 8; };
$lc = "hello world";  # line 30
print "After 'typeset -l lc=\"HELLO WORLD\"': lc='" . ($lc) . "'\n";  # line 31
$? = 0 << 8;
$lc = "another test";
print "After 'lc=\"ANOTHER TEST\"':           lc='" . ($lc) . "'\n";  # line 33
$? = 0 << 8;
print "\n";  # line 34
$? = 0 << 8;
print "=== typeset -u (uppercase attribute) ===\n";
$? = 0 << 8;
do { { no strict 'refs'; undef ${'uc'}; undef @{'uc'}; undef %{'uc'}; delete $ENV{'uc'}; }  # line 38
$? = 0 << 8; };
$uc = "HELLO WORLD";  # line 39
print "After 'typeset -u uc=\"hello world\"': uc='" . ($uc) . "'\n";  # line 40
$? = 0 << 8;
$uc = "ANOTHER TEST";
print "After 'uc=\"another test\"':            uc='" . ($uc) . "'\n";  # line 42
$? = 0 << 8;
print "\n";  # line 43
$? = 0 << 8;
print "=== typeset -x (export attribute) ===\n";
$? = 0 << 8;
do { { no strict 'refs'; undef ${'myexport'}; undef @{'myexport'}; undef %{'myexport'}; delete $ENV{'myexport'}; }  # line 47
$? = 0 << 8; };
do { $myexport = 'exported_value'; $ENV{myexport} = 'exported_value';  # line 48
$? = 0 << 8; };
print "After 'typeset -x myexport=exported_value'\n";
$? = 0 << 8;
print "Variable is exported:\n";
$? = 0 << 8;
do { my @__m = grep { /^myexport=/ } map { "$_=$ENV{$_}" } sort keys %ENV; if (@__m) { print STDOUT join("\n", @__m), "\n"; $? = 0 << 8; } else { print STDOUT '(myexport not found in env — possible scope issue)', "\n"; $? = 1 << 8; } };print "\n";  # line 53
$? = 0 << 8;
print "=== typeset -a (indexed array) ===\n";
$? = 0 << 8;
do { { no strict 'refs'; undef ${'arr'}; undef @{'arr'}; undef %{'arr'}; delete $ENV{'arr'}; }  # line 57
$? = 0 << 8; };
@arr = ("10", "20", "30");  # line 58
print("After 'typeset -a arr=(10 20 30)': arr=(" . (join(' ', @arr)) . ")", "\n");  # line 59
print("arr[0]='" . ($arr[0]) . "' arr[1]='" . ($arr[1]) . "' arr[2]='" . ($arr[2]) . "'", "\n");  # line 60
print "\n";  # line 61
$? = 0 << 8;
print "=== typeset -A (associative array) ===\n";
$? = 0 << 8;
do { { no strict 'refs'; undef ${'assoc'}; undef @{'assoc'}; undef %{'assoc'}; delete $ENV{'assoc'}; }  # line 65
$? = 0 << 8; };
%assoc = ("key1", "value1", "key2", "value2");  # line 66
print "After 'typeset -A assoc=([key1]=value1 [key2]=value2)'\n";
$? = 0 << 8;
print("assoc[key1]='" . ($assoc{'key1'}) . "'  assoc[key2]='" . ($assoc{'key2'}) . "'", "\n");  # line 68
print "\n";  # line 69
$? = 0 << 8;
print "=== typeset -n (name reference) ===\n";
$? = 0 << 8;
do { { no strict 'refs'; undef ${'original'}; undef @{'original'}; undef %{'original'}; delete $ENV{'original'}; undef ${'ref'}; undef @{'ref'}; undef %{'ref'}; delete $ENV{'ref'}; }  # line 73
$? = 0 << 8; };
$original = "I am the original";
$? = 0 << 8;  # line 75
print "After 'typeset -n ref=original':\n";
$? = 0 << 8;
print "original='" . ($original) . "'\n";  # line 77
$? = 0 << 8;
print "ref='" . ($original) . "'\n";  # line 78
$? = 0 << 8;
$original = "Changed via ref";
print "After 'ref=\"Changed via ref\"':\n";
$? = 0 << 8;
print "original='" . ($original) . "'\n";  # line 81
$? = 0 << 8;
print "ref='" . ($original) . "'\n";  # line 82
$? = 0 << 8;
$? = 0 << 8;  # line 83
print "\n";  # line 84
$? = 0 << 8;
print "=== typeset -f (display function definition) ===\n";
$? = 0 << 8;
sub myfunc {
    local @ARGV = @_;
    my $__argc = @ARGV;
    print "Inside myfunc\n";
    $? = 0 << 8;
    my $x = "5";
    say "x=$x";
    $? = 0 << 8;
}
print "Output of 'typeset -f myfunc':\n";
$? = 0 << 8;
print("myfunc () \n{ \n    echo \"Inside myfunc\";\n    local x=5;\n    echo \"x=\$x\"\n}\n");  # line 94
$? = 0 << 8;
print "\n";  # line 95
$? = 0 << 8;
print "=== typeset -F (list function names) ===\n";
$? = 0 << 8;
print("myfunc\n");  # line 99
$? = 0 << 8;
print "\n";  # line 100
$? = 0 << 8;
print "=== typeset -g (global scope in function) ===\n";
$? = 0 << 8;
sub set_global {  # line 104
    local @ARGV = @_;
    my $__argc = @ARGV;
    $global_var = "I am global";
    $? = 0 << 8;
}
set_global();  # line 107
print "After set_global(): global_var='" . ($global_var) . "'\n";  # line 108
$? = 0 << 8;
print "\n";  # line 109
$? = 0 << 8;
print "=== typeset -t (trace attribute) ===\n";
$? = 0 << 8;
$tracetest = "traced";  # line 113
print "After 'typeset -t tracetest=traced': (trace attribute set)\n";
$? = 0 << 8;
print "tracetest='" . ($tracetest) . "'\n";  # line 115
$? = 0 << 8;
print "\n";  # line 116
$? = 0 << 8;
print "=== typeset -p (print attribute info) ===\n";
$? = 0 << 8;
do { { no strict 'refs'; undef ${'printtest'}; undef @{'printtest'}; undef %{'printtest'}; delete $ENV{'printtest'}; }  # line 120
$? = 0 << 8; };
do { $printtest = '99';  # line 121
$? = 0 << 8; };
print "After 'typeset -i -r printtest=99':\n";
$? = 0 << 8;
print("declare -ir printtest=\"99\"\n");  # line 124
$? = 0 << 8;
print "\n";  # line 125
$? = 0 << 8;
print "=== combined: typeset -il (integer + lowercase) ===\n";
$? = 0 << 8;
do { { no strict 'refs'; undef ${'comb'}; undef @{'comb'}; undef %{'comb'}; delete $ENV{'comb'}; }  # line 129
$? = 0 << 8; };
do { $comb = lc('42');  # line 130
$? = 0 << 8; };
print "After 'typeset -il comb=42': comb='" . ($comb) . "' (integer + lowercase)\n";  # line 131
$? = 0 << 8;
$comb = lc(int(($comb + 1)));
print "After 'comb=comb+1':    comb='" . ($comb) . "' (integer arithmetic active)\n";  # line 133
$? = 0 << 8;
print "\n";  # line 134
$? = 0 << 8;
print "=== combined: typeset -iu (integer + uppercase) ===\n";
$? = 0 << 8;
do { { no strict 'refs'; undef ${'comb2'}; undef @{'comb2'}; undef %{'comb2'}; delete $ENV{'comb2'}; }  # line 137
$? = 0 << 8; };
do { $comb2 = uc('99');  # line 138
$? = 0 << 8; };
print "After 'typeset -iu comb2=99': comb2='" . ($comb2) . "' (integer + uppercase)\n";  # line 139
$? = 0 << 8;
$comb2 = uc(int(($comb2 + 1)));
print "After 'comb2=comb2+1': comb2='" . ($comb2) . "' (integer arithmetic active)\n";  # line 141
$? = 0 << 8;
print "\n";  # line 142
$? = 0 << 8;
print "=== typeset -a (indexed array, individual element) ===\n";
$? = 0 << 8;
do { { no strict 'refs'; undef ${'singlearr'}; undef @{'singlearr'}; undef %{'singlearr'}; delete $ENV{'singlearr'}; }  # line 146
$? = 0 << 8; };
@singlearr = ();  # line 147
$singlearr[0] = "first";
$singlearr[1] = "second";
print("singlearr[0]='" . ($singlearr[0]) . "'  singlearr[1]='" . ($singlearr[1]) . "'", "\n");  # line 150
print "\n";  # line 151
$? = 0 << 8;
print "=== typeset (no flag) ===\n";
$? = 0 << 8;
do { { no strict 'refs'; undef ${'plain'}; undef @{'plain'}; undef %{'plain'}; delete $ENV{'plain'}; }  # line 155
$? = 0 << 8; };
$plain = "just a string";  # line 156
$? = 0 << 8;
print "After 'typeset plain=\"just a string\"': plain='" . ($plain) . "'\n";  # line 157
$? = 0 << 8;
print "\n";  # line 158
$? = 0 << 8;
print "=== Demonstration complete. ===\n";
$? = 0 << 8;

exit $? >> 8;
