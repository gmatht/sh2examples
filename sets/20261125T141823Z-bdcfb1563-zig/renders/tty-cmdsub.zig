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

fn sh2TestStub() bool {
    sh2TODO("test");
    return false;
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
    } else {
        try stdout.print("{s}\n", .{"  stdout  : (empty)"});
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

    var TTY_DEV: []const u8 = "";
    var dev: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    _ = sh2Run(&[_][]const u8{"set", "-u"}); // line 16
    TTY_DEV = "";
    { // line 54
        const sh2_items0 = [_][]const u8{"/dev/pts/2", "/dev/pts/3", "/dev/pts/4"};
        for (sh2_items0[0..]) |sh2_v1| {
            dev = sh2_v1;
            if (sh2Test()) {
                TTY_DEV = sh2IntStr(dev);
                break;
            }
        }
    }
    if ((!!std.mem.eql(u8, TTY_DEV, ""))) {
        {
            const sh2_items2 = [_][]const u8{"\x01SH2GLOB\x01/dev/pts/*"};
            for (sh2_items2[0..]) |sh2_v3| {
                dev = sh2_v3;
                if (sh2Test()) {
                    TTY_DEV = sh2IntStr(dev);
                    break;
                }
            }
        }
    }
    // TODO(unsupported): param → sh2.param // line 70
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"Using terminal device: ", sh2Param()});
    try stdout.flush();
    try stdout.print("{s}\n", .{""});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 1: tty (default) \xe2\x80\x94 with a real terminal"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    if (!std.mem.eql(u8, TTY_DEV, "")) {
        _ = sh2Run(&[_][]const u8{"capture", "01-default-terminal", "tty"});
    } else {
        try stdout.print("{s}\n", .{"  (skipped \xe2\x80\x94 no terminal device available)"});
    try stdout.flush();
        try stdout.print("{s}\n", .{""});
    try stdout.flush();
    }
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 2: tty (default) \xe2\x80\x94 stdin from /dev/null"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "02-not-a-tty", "tty"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 3: tty (default) \xe2\x80\x94 piped input"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "03-pipe-notty", "bash", "-c", "echo \"dummy\" | tty"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 4: tty -s \xe2\x80\x94 with a real terminal (silent)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    if (!std.mem.eql(u8, TTY_DEV, "")) {
        _ = sh2Run(&[_][]const u8{"capture", "04-silent-terminal", "tty", "-s"});
    } else {
        try stdout.print("{s}\n", .{"  (skipped \xe2\x80\x94 no terminal device available)"});
    try stdout.flush();
        try stdout.print("{s}\n", .{""});
    try stdout.flush();
    }
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 5: tty -s \xe2\x80\x94 stdin from /dev/null (silent, not a tty)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "05-silent-notty", "tty", "-s"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 6: tty -s \xe2\x80\x94 piped input (silent, not a tty)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "06-silent-pipe", "bash", "-c", "echo \"dummy\" | tty -s"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 7: tty --silent \xe2\x80\x94 long form, with a real terminal"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    if (!std.mem.eql(u8, TTY_DEV, "")) {
        _ = sh2Run(&[_][]const u8{"capture", "07-long-silent", "tty", "--silent"});
    } else {
        try stdout.print("{s}\n", .{"  (skipped \xe2\x80\x94 no terminal device available)"});
    try stdout.flush();
        try stdout.print("{s}\n", .{""});
    try stdout.flush();
    }
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 8: tty --quiet \xe2\x80\x94 long form, with a real terminal"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    if (!std.mem.eql(u8, TTY_DEV, "")) {
        _ = sh2Run(&[_][]const u8{"capture", "08-long-quiet", "tty", "--quiet"});
    } else {
        try stdout.print("{s}\n", .{"  (skipped \xe2\x80\x94 no terminal device available)"});
    try stdout.flush();
        try stdout.print("{s}\n", .{""});
    try stdout.flush();
    }
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 9: tty --silent \xe2\x80\x94 stdin from /dev/null (not a tty)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "09-long-silent-notty", "tty", "--silent"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 10: tty --quiet \xe2\x80\x94 stdin from /dev/null (not a tty)"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "10-long-quiet-notty", "tty", "--quiet"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 11: tty (default) \xe2\x80\x94 inheriting stdin from the script"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "11-inherited", "tty"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 12: tty -s \xe2\x80\x94 inheriting stdin from the script"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"capture", "12-inherited-silent", "tty", "-s"});
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    try stdout.print("{s}\n", .{" SECTION 13: tty -s -s \xe2\x80\x94 repeated silent flag"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"============================================================"});
    try stdout.flush();
    if (!std.mem.eql(u8, TTY_DEV, "")) {
        _ = sh2Run(&[_][]const u8{"capture", "13-double-silent", "tty", "-s", "-s"});
    } else {
        try stdout.print("{s}\n", .{"  (skipped \xe2\x80\x94 no terminal device available)"});
    try stdout.flush();
        try stdout.print("{s}\n", .{""});
    try stdout.flush();
    }
    try stdout.print("{s}\n", .{""});
    try stdout.flush();
    try stdout.print("{s}\n", .{"All tty demo sections completed."});
    try stdout.flush();
}
// 12 construct(s) lowered to TODO markers