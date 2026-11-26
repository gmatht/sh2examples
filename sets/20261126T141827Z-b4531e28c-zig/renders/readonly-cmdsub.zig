const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2IntStr(n: i64) []const u8 {
    return std.fmt.allocPrint(std.heap.page_allocator, "{d}", .{n}) catch "";
}

var sh2_io: ?std.Io = null;
var sh2_environ: ?*std.process.Environ.Map = null;

fn sh2Env(name: []const u8) []const u8 {
    const m = sh2_environ orelse return "";
    return m.get(name) orelse "";
}

fn sh2WriteFile(p: []const u8, data: []const u8, append: bool) void {
    const f = std.fs.cwd().createFile(p, .{ .truncate = !append }) catch return;
    defer f.close();
    f.writeAll(data) catch {};
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

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}


fn capture(init: std.process.Init, __args: []const []const u8) ![]const u8 {
    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    var se: []const u8 = "";
    var so: []const u8 = "";
    var tmp_stderr: []const u8 = "";
    var tmp_stdout: []const u8 = "";
    _ = sh2Run(&[_][]const u8{"local", "label=", (if (__args.len >= 1) __args[1 - 1] else "")});
    _ = sh2Run(&[_][]const u8{"shift"});
    // declare tmp_stdout
    // declare tmp_stderr
    tmp_stdout = sh2Capture();
    tmp_stderr = sh2Capture();
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = sh2Exec();
    sh2WriteFile(sh2IntStr(tmp_stdout), "", false);
    _ = sh2Run(&[_][]const u8{"local", "ec=", sh2Env("?")});
    // declare so
    so = sh2Capture();
    // declare se
    se = sh2Capture();
    _ = sh2Run(&[_][]const u8{"rm", "-f", sh2IntStr(tmp_stdout), sh2IntStr(tmp_stderr)});
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}{s}\n", .{"--- [", sh2Param(), "] ---"});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"  cmd     : ", sh2Env("*")});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"  exitcode: ", sh2Param()});
    try stdout.flush();
    if (!std.mem.eql(u8, so, "")) {
        // TODO(unsupported): param → sh2.param
        // TODO(unsupported): param → sh2.param
        try stdout.print("{s}{s}\n", .{"  stdout  : ", sh2Param()});
    try stdout.flush();
    }
    if (!std.mem.eql(u8, se, "")) {
        // TODO(unsupported): param → sh2.param
        // TODO(unsupported): param → sh2.param
        try stdout.print("{s}{s}\n", .{"  stderr  : ", sh2Param()});
    try stdout.flush();
    }
    try stdout.print("{s}\n", .{""});
    try stdout.flush();
    return sh2Env("ec");
}
pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    _ = sh2Run(&[_][]const u8{"set", "-u"}); // line 13
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 1: Basic readonly variable"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "01-readonly-var", "bash", "-c", "\n        readonly MY_VAR=\"hello_world\"\n        echo \"MY_VAR=${MY_VAR}\"\n        # Attempt to change it (will fail, but we capture)\n        MY_VAR=\"changed\" 2>/dev/null || echo \"Assignment denied: $?\"\n        echo \"MY_VAR after attempted change=${MY_VAR}\"\n    "});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 2: readonly -p  (print readonly variables)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "02-readonly-p", "bash", "-c", "\n        readonly FOO=alpha\n        readonly BAR=beta\n        readonly -p | head -5\n    "});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 3: readonly -f  (readonly functions)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "03-readonly-f", "bash", "-c", "\n        myfunc() { echo \"Hello from myfunc\"; }\n        readonly -f myfunc\n        myfunc\n        # Attempt to unset the function (will fail)\n        unset -f myfunc 2>/dev/null || echo \"unset -f denied\"\n        # Verify function still exists\n        myfunc\n    "});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 4: readonly -a  (indexed array)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "04-readonly-a", "bash", "-c", "\n        declare -a MY_ARR=(zero one two)\n        readonly -a MY_ARR\n        echo \"MY_ARR[0]=${MY_ARR[0]}\"\n        echo \"MY_ARR[1]=${MY_ARR[1]}\"\n        # Attempt to change an element (will fail)\n        MY_ARR[0]=ZERO 2>/dev/null || echo \"Array assignment denied\"\n        echo \"MY_ARR[0] after attempt=${MY_ARR[0]}\"\n    "});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 5: readonly -A  (associative array)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "05-readonly-A", "bash", "-c", "\n        declare -A MY_ASSOC=([key1]=val1 [key2]=val2)\n        readonly -A MY_ASSOC\n        echo \"MY_ASSOC[key1]=${MY_ASSOC[key1]}\"\n        echo \"MY_ASSOC[key2]=${MY_ASSOC[key2]}\"\n        # Attempt to change (will fail)\n        MY_ASSOC[key1]=CHANGED 2>/dev/null || echo \"Assoc assignment denied\"\n        echo \"MY_ASSOC[key1] after attempt=${MY_ASSOC[key1]}\"\n    "});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 6: readonly --  (no more options)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "06a-readonly-dash-ok", "bash", "-c", "\n        readonly -- MYVAR=hello\n        echo \"MYVAR=${MYVAR}\"\n        # Confirm it is indeed readonly\n        MYVAR=world 2>/dev/null || echo \"Assignment denied (expected)\"\n    "});
    _ = sh2Run(&[_][]const u8{"capture", "06b-readonly-dash-invalid", "bash", "-c", "\n        readonly -- -notvalid 2>&1 || true\n    "});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 7: Invalid variable name (error case)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "07-invalid-name", "bash", "-c", "\n        readonly 123invalid 2>&1 || true\n    "});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 8: readonly -f -p  (list readonly functions)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "08-readonly-f-p", "bash", "-c", "\n        f1() { :; }\n        f2() { :; }\n        readonly -f f1 f2\n        readonly -f -p | head -5\n    "});
    try stdout.print("{s}\n", .{""});
    try stdout.flush();
    try stdout.print("{s}\n", .{"All readonly demo sections completed."});
    try stdout.flush();
    std.process.exit(@intCast(0));
}
// 10 construct(s) lowered to TODO markers