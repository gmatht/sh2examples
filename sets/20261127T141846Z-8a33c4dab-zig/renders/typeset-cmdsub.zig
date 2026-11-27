const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

var sh2_io: ?std.Io = null;
var sh2_environ: ?*std.process.Environ.Map = null;

fn sh2Env(name: []const u8) []const u8 {
    const m = sh2_environ orelse return "";
    return m.get(name) orelse "";
}

fn sh2Run(argv: []const []const u8) []const u8 {
    const io = sh2_io orelse return "";
    const result = std.process.run(std.heap.page_allocator, io, .{
        .argv = argv,
        .stdout_limit = .limited(1 << 20),
    }) catch return "";
    return std.mem.trimEnd(u8, result.stdout, "\n");
}

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

fn sh2Int() []const u8 {
    sh2TODO("int");
    return "";
}

fn sh2Join() []const u8 {
    sh2TODO("join");
    return "";
}

fn sh2Lc() []const u8 {
    sh2TODO("lc");
    return "";
}

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}

fn sh2Pipeline() []const u8 {
    sh2TODO("pipeline");
    return "";
}

fn sh2Uc() []const u8 {
    sh2TODO("uc");
    return "";
}


fn myfunc(init: std.process.Init, __args: []const []const u8) ![]const u8 {
    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    _ = __args;
    var x: []const u8 = "";
    try stdout.print("{s}\n", .{"Inside myfunc"});
    try stdout.flush();
    x = "5";
    try stdout.print("{s}{s}\n", .{"x=", x});
    try stdout.flush();
}
fn set_global(init: std.process.Init, __args: []const []const u8) []const u8 {
    _ = init;
    _ = __args;
    _ = sh2Run(&[_][]const u8{"typeset", "-g", "global_var=", "I am global"});
}
pub fn main(init: std.process.Init) !void {

    var comb: []const u8 = "";
    var comb2: []const u8 = "";
    var lc: []const u8 = "";
    var n: []const u8 = "";
    var original: []const u8 = "";
    var ref: []const u8 = "";
    var singlearr_0_: []const u8 = "";
    var singlearr_1_: []const u8 = "";
    var uc: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    try stdout.print("{s}\n", .{"=== typeset -i (integer attribute) ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"unset", "n"}); // line 10
    _ = sh2Run(&[_][]const u8{"typeset", "-i", "n=42"}); // line 11
    try stdout.print("{s}{s}{s}\n", .{"After 'typeset -i n=42': n='", n, "'"});
    try stdout.flush(); // line 12
    // TODO(unsupported): int → sh2.int
    // TODO(unsupported): int → sh2.int
    n = sh2Int();
    try stdout.print("{s}{s}{s}\n", .{"After 'n=n+1':          n='", n, "' (integer arithmetic applied)"});
    try stdout.flush(); // line 14
    // TODO(unsupported): int → sh2.int
    // TODO(unsupported): int → sh2.int
    n = sh2Int();
    try stdout.print("{s}{s}{s}\n", .{"After 'n=\"hello\"':     n='", n, "' (assigns 0; non-numeric string becomes 0)"});
    try stdout.flush(); // line 16
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 17
    try stdout.print("{s}\n", .{"=== typeset -r (readonly attribute) ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"unset", "rovar"}); // line 21
    _ = sh2Run(&[_][]const u8{"typeset", "-r", "rovar=", "immutable"}); // line 22
    try stdout.print("{s}{s}{s}\n", .{"After 'typeset -r rovar=immutable': rovar='", sh2Env("rovar"), "'"});
    try stdout.flush(); // line 23
    try stdout.print("{s}\n", .{"(Attempting 'rovar=change' would cause an error; skipped for safety.)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 25
    try stdout.print("{s}\n", .{"=== typeset -l (lowercase attribute) ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"unset", "lc"}); // line 29
    _ = sh2Run(&[_][]const u8{"typeset", "-l", "lc=", "HELLO WORLD"}); // line 30
    try stdout.print("{s}{s}{s}\n", .{"After 'typeset -l lc=\"HELLO WORLD\"': lc='", lc, "'"});
    try stdout.flush(); // line 31
    lc = "another test";
    try stdout.print("{s}{s}{s}\n", .{"After 'lc=\"ANOTHER TEST\"':           lc='", lc, "'"});
    try stdout.flush(); // line 33
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 34
    try stdout.print("{s}\n", .{"=== typeset -u (uppercase attribute) ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"unset", "uc"}); // line 38
    _ = sh2Run(&[_][]const u8{"typeset", "-u", "uc=", "hello world"}); // line 39
    try stdout.print("{s}{s}{s}\n", .{"After 'typeset -u uc=\"hello world\"': uc='", uc, "'"});
    try stdout.flush(); // line 40
    uc = "ANOTHER TEST";
    try stdout.print("{s}{s}{s}\n", .{"After 'uc=\"another test\"':            uc='", uc, "'"});
    try stdout.flush(); // line 42
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 43
    try stdout.print("{s}\n", .{"=== typeset -x (export attribute) ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"unset", "myexport"}); // line 47
    _ = sh2Run(&[_][]const u8{"typeset", "-x", "myexport=", "exported_value"}); // line 48
    try stdout.print("{s}\n", .{"After 'typeset -x myexport=exported_value'"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"Variable is exported:"});
    try stdout.flush();
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Pipeline()) or sh2Truthy(sh2Exec()));
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 53
    try stdout.print("{s}\n", .{"=== typeset -a (indexed array) ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"unset", "arr"}); // line 57
    _ = sh2Run(&[_][]const u8{"typeset", "-a", [_] []const u8{"10", "20", "30"}}); // line 58
    // TODO(unsupported): join → sh2.join // line 59
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}{s}\n", .{"After 'typeset -a arr=(10 20 30)': arr=(", sh2Join(), ")"});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 60
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}{s}{s}{s}{s}{s}\n", .{"arr[0]='", sh2Param(), "' arr[1]='", sh2Param(), "' arr[2]='", sh2Param(), "'"});
    try stdout.flush();
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 61
    try stdout.print("{s}\n", .{"=== typeset -A (associative array) ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"unset", "assoc"}); // line 65
    _ = sh2Run(&[_][]const u8{"typeset", "-A", [_] []const u8{"[key1]=value1", "[key2]=value2"}}); // line 66
    try stdout.print("{s}\n", .{"After 'typeset -A assoc=([key1]=value1 [key2]=value2)'"});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 68
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}{s}{s}{s}\n", .{"assoc[key1]='", sh2Param(), "'  assoc[key2]='", sh2Param(), "'"});
    try stdout.flush();
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 69
    try stdout.print("{s}\n", .{"=== typeset -n (name reference) ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"unset", "original", "ref"}); // line 73
    original = "I am the original";
    _ = sh2Run(&[_][]const u8{"typeset", "-n", "ref=original"}); // line 75
    try stdout.print("{s}\n", .{"After 'typeset -n ref=original':"});
    try stdout.flush();
    try stdout.print("{s}{s}{s}\n", .{"original='", original, "'"});
    try stdout.flush(); // line 77
    try stdout.print("{s}{s}{s}\n", .{"ref='", ref, "'"});
    try stdout.flush(); // line 78
    ref = "Changed via ref";
    try stdout.print("{s}\n", .{"After 'ref=\"Changed via ref\"':"});
    try stdout.flush();
    try stdout.print("{s}{s}{s}\n", .{"original='", original, "'"});
    try stdout.flush(); // line 81
    try stdout.print("{s}{s}{s}\n", .{"ref='", ref, "'"});
    try stdout.flush(); // line 82
    _ = sh2Run(&[_][]const u8{"unset", "-n", "ref"}); // line 83
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 84
    try stdout.print("{s}\n", .{"=== typeset -f (display function definition) ==="});
    try stdout.flush();
    try stdout.print("{s}\n", .{"Output of 'typeset -f myfunc':"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"typeset", "-f", "myfunc"}); // line 94
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 95
    try stdout.print("{s}\n", .{"=== typeset -F (list function names) ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"typeset", "-F", "myfunc"}); // line 99
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 100
    try stdout.print("{s}\n", .{"=== typeset -g (global scope in function) ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"set_global"}); // line 107
    try stdout.print("{s}{s}{s}\n", .{"After set_global(): global_var='", sh2Env("global_var"), "'"});
    try stdout.flush(); // line 108
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 109
    try stdout.print("{s}\n", .{"=== typeset -t (trace attribute) ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"typeset", "-t", "tracetest=traced"}); // line 113
    try stdout.print("{s}\n", .{"After 'typeset -t tracetest=traced': (trace attribute set)"});
    try stdout.flush();
    try stdout.print("{s}{s}{s}\n", .{"tracetest='", sh2Env("tracetest"), "'"});
    try stdout.flush(); // line 115
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 116
    try stdout.print("{s}\n", .{"=== typeset -p (print attribute info) ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"unset", "printtest"}); // line 120
    _ = sh2Run(&[_][]const u8{"typeset", "-i", "printtest=99"}); // line 121
    _ = sh2Run(&[_][]const u8{"typeset", "-r", "printtest"}); // line 122
    try stdout.print("{s}\n", .{"After 'typeset -i -r printtest=99':"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"typeset", "-p", "printtest"}); // line 124
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 125
    try stdout.print("{s}\n", .{"=== combined: typeset -il (integer + lowercase) ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"unset", "comb"}); // line 129
    _ = sh2Run(&[_][]const u8{"typeset", "-il", "comb=42"}); // line 130
    try stdout.print("{s}{s}{s}\n", .{"After 'typeset -il comb=42': comb='", comb, "' (integer + lowercase)"});
    try stdout.flush(); // line 131
    // TODO(unsupported): lc → sh2.lc
    // TODO(unsupported): lc → sh2.lc
    comb = sh2Lc();
    try stdout.print("{s}{s}{s}\n", .{"After 'comb=comb+1':    comb='", comb, "' (integer arithmetic active)"});
    try stdout.flush(); // line 133
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 134
    try stdout.print("{s}\n", .{"=== combined: typeset -iu (integer + uppercase) ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"unset", "comb2"}); // line 137
    _ = sh2Run(&[_][]const u8{"typeset", "-iu", "comb2=99"}); // line 138
    try stdout.print("{s}{s}{s}\n", .{"After 'typeset -iu comb2=99': comb2='", comb2, "' (integer + uppercase)"});
    try stdout.flush(); // line 139
    // TODO(unsupported): uc → sh2.uc
    // TODO(unsupported): uc → sh2.uc
    comb2 = sh2Uc();
    try stdout.print("{s}{s}{s}\n", .{"After 'comb2=comb2+1': comb2='", comb2, "' (integer arithmetic active)"});
    try stdout.flush(); // line 141
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 142
    try stdout.print("{s}\n", .{"=== typeset -a (indexed array, individual element) ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"unset", "singlearr"}); // line 146
    _ = sh2Run(&[_][]const u8{"typeset", "-a", "singlearr"}); // line 147
    singlearr_0_ = "first";
    singlearr_1_ = "second";
    // TODO(unsupported): param → sh2.param // line 150
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}{s}{s}{s}\n", .{"singlearr[0]='", sh2Param(), "'  singlearr[1]='", sh2Param(), "'"});
    try stdout.flush();
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 151
    try stdout.print("{s}\n", .{"=== typeset (no flag) ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"unset", "plain"}); // line 155
    _ = sh2Run(&[_][]const u8{"typeset", "plain=", "just a string"}); // line 156
    try stdout.print("{s}{s}{s}\n", .{"After 'typeset plain=\"just a string\"': plain='", sh2Env("plain"), "'"});
    try stdout.flush(); // line 157
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 158
    try stdout.print("{s}\n", .{"=== Demonstration complete. ==="});
    try stdout.flush();
    _ = &singlearr_0_;
    _ = &singlearr_1_;
}
// 28 construct(s) lowered to TODO markers