const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2ToInt(s: []const u8) i64 {
    return std.fmt.parseInt(i64, std.mem.trim(u8, s, " \t\n\r"), 10) catch 0;
}

var sh2_io: ?std.Io = null;
var sh2_environ: ?*std.process.Environ.Map = null;

fn sh2Env(name: []const u8) []const u8 {
    const m = sh2_environ orelse return "";
    return m.get(name) orelse "";
}

fn sh2FileTest(flag: []const u8, p: []const u8) bool {
    const st = std.fs.cwd().statFile(p) catch return false;
    if (std.mem.eql(u8, flag, "-f")) return st.kind == .file;
    if (std.mem.eql(u8, flag, "-d")) return st.kind == .directory;
    if (std.mem.eql(u8, flag, "-e")) return true;
    if (std.mem.eql(u8, flag, "-s")) return st.size != 0;
    return false;
}

fn sh2Run(argv: []const []const u8) []const u8 {
    const io = sh2_io orelse return "";
    const result = std.process.run(std.heap.page_allocator, io, .{
        .argv = argv,
        .stdout_limit = .limited(1 << 20),
    }) catch return "";
    return std.mem.trimEnd(u8, result.stdout, "\n");
}

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

fn sh2Join() []const u8 {
    sh2TODO("join");
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


fn complex_function(init: std.process.Init, __args: []const []const u8) ![]const u8 {
    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    _ = sh2Run(&[_][]const u8{"local", "param1=", (if (__args.len >= 1) __args[1 - 1] else "")});
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    _ = sh2Run(&[_][]const u8{"local", "param2=", sh2Param()});
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    _ = sh2Run(&[_][]const u8{"local", "param3=", sh2Param()});
    try stdout.print("{s}{s}\n", .{"Param1: ", sh2Env("param1")});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"Param2: ", sh2Env("param2")});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"Param3: ", sh2Env("param3")});
    try stdout.flush();
    // TODO(unsupported): capture → sh2.capture
    // TODO(unsupported): capture → sh2.capture
    _ = sh2Run(&[_][]const u8{"local", "result=", sh2Capture()});
    try stdout.print("{s}{d}\n", .{"Result: ", result});
    try stdout.flush();
}
fn test_locals(init: std.process.Init, __args: []const []const u8) ![]const u8 {
    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    _ = sh2Run(&[_][]const u8{"local", "var1=", (if (__args.len >= 1) __args[1 - 1] else "")});
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    _ = sh2Run(&[_][]const u8{"local", "var2=", sh2Param()});
    _ = sh2Run(&[_][]const u8{"local", "var3=", sh2Capture()});
    try stdout.print("{s}{s}\n", .{"Var1: ", sh2Env("var1")});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"Var2: ", sh2Env("var2")});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"Var3: ", sh2Env("var3")});
    try stdout.flush();
}
pub fn main(init: std.process.Init) !void {

    var array: [1024]i64 = undefined;
    var array_len: usize = 0;
    var complex_var: []const u8 = "";
    var message: []const u8 = "";
    var result: i64 = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    try stdout.print("{s}\n", .{"Testing ambiguous operators..."});
    try stdout.flush();
    result = 512;
    try stdout.print("{s}{d}\n", .{"2**3**2 = ", result});
    try stdout.flush(); // line 10
    try stdout.print("{s}\n", .{"Testing complex parameter expansions..."});
    try stdout.flush();
    complex_var = "hello world";
    // TODO(unsupported): param → sh2.param // line 15
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 16
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 17
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 18
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{sh2Param()});
    try stdout.flush();
    try stdout.print("{s}\n", .{"Testing complex here-documents..."});
    try stdout.flush();
    try stdout.print("{s}", .{"This is a test line\nThis is not a test line\nThis is another test line\n"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"Testing nested arithmetic..."});
    try stdout.flush();
    result = 40;
    try stdout.print("{s}{d}\n", .{"Complex arithmetic: ", result});
    try stdout.flush(); // line 31
    try stdout.print("{s}\n", .{"Testing nested command substitution..."});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 35
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"Current dir: ", sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 36
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"User: ", sh2Param()});
    try stdout.flush();
    try stdout.print("{s}\n", .{"Testing process substitution..."});
    try stdout.flush();
    try stdout.print("{s}\n", .{"Testing complex brace expansion..."});
    try stdout.flush();
    try stdout.print("{s}\n", .{"a1x a1y a1z a2x a2y a2z a3x a3y a3z b1x b1y b1z b2x b2y b2z b3x b3y b3z c1x c1y c1z c2x c2y c2z c3x c3y c3z"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"Testing simple case patterns..."});
    try stdout.flush();
    if ((!!std.mem.eql(u8, sh2Env("1"), "test"))) {
        try stdout.print("{s}\n", .{"Matched test"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"Default case"});
    try stdout.flush();
    }
    try stdout.print("{s}\n", .{"Testing simple pipeline..."});
    try stdout.flush();
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    try stdout.print("{s}\n", .{"Testing mixed arithmetic..."});
    try stdout.flush();
    result = 776;
    try stdout.print("{s}{d}\n", .{"Mixed base result: ", result});
    try stdout.flush(); // line 82
    try stdout.print("{s}\n", .{"Testing complex string interpolation..."});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 86
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    message = std.fmt.allocPrint(std.heap.page_allocator, "Hello, {s}! Your home is {s}", .{sh2Param(), sh2Param()}) catch "";
    try stdout.print("{s}\n", .{message});
    try stdout.flush(); // line 87
    try stdout.print("{s}\n", .{"Testing simple test expressions..."});
    try stdout.flush();
    if (sh2FileTest("-f", "file.txt")) {
        try stdout.print("{s}\n", .{"File exists"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"File does not exist"});
    try stdout.flush();
    }
    try stdout.print("{s}\n", .{"Testing complex array operations..."});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"declare", "-a", [_] []const u8{"item1", "item2", "item3"}});
    array[array_len] = sh2ToInt("item4");
    array_len += 1;
    // TODO(unsupported): join → sh2.join // line 101
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}\n", .{"Array: ", sh2Join()});
    try stdout.flush();
    // TODO(unsupported): join → sh2.join
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}\n", .{"Length: ", sh2Join()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 103
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"First item: ", sh2Param()});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"complex_function", "test\"quote", "second_param", "third\"param"});
    _ = sh2Run(&[_][]const u8{"test_locals", "hello", "world"});
    _ = &array;
    _ = &complex_var;
}
// 32 construct(s) lowered to TODO markers