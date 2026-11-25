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

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

fn sh2Pipeline() []const u8 {
    sh2TODO("pipeline");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var d: []const u8 = "";
    var matched: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    try stdout.print("{s}{s}{s}\n", .{"-e", " ", "line1\\nline2\\nTARGET\\nline4\\nline5"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"grep", "-A", "2", "TARGET"});
    try stdout.print("{s}{s}{s}\n", .{"-e", " ", "line1\\nline2\\nTARGET\\nline4\\nline5"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"grep", "-B", "2", "TARGET"});
    try stdout.print("{s}{s}{s}\n", .{"-e", " ", "line1\\nline2\\nTARGET\\nline4\\nline5"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"grep", "-C", "1", "TARGET"});
    d = sh2Capture(); // line 12
    // TODO(unsupported): exec → sh2.exec // line 13
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Exec()) or sh2Truthy(sh2Exec()));
    try std.fs.cwd().writeFile(.{ .sub_path = "temp_file1.txt", .data = "pattern in file1\n" });
    try std.fs.cwd().writeFile(.{ .sub_path = "temp_file2.txt", .data = "no pattern in file2\n" });
    try std.fs.cwd().writeFile(.{ .sub_path = "temp_file3.txt", .data = "pattern in file3\n" });
    try stdout.print("{s}\n", .{"Recursive search results:"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"grep", "-r", "pattern", ".", "\x01SH2GLOB\x01--include=*.txt"});
    try stdout.print("{s}\n", .{"Result 2..."});
    try stdout.flush(); // line 21
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    try stdout.print("{s}\n", .{"Result 3..."});
    try stdout.flush(); // line 25
    _ = sh2Run(&[_][]const u8{"grep", "-L", "pattern", "\x01SH2GLOB\x01*.txt"});
    _ = sh2Run(&[_][]const u8{"cd", "/"}); // line 29
    _ = sh2Run(&[_][]const u8{"rm", "-r", "-f", d}); // line 30
    matched = sh2Capture();
    try stdout.print("{s}{s}\n", .{"  grep_exit: ", sh2Env("?")});
    try stdout.flush(); // line 33
    try stdout.print("{s}{s}\n", .{"  match_count: ", matched});
    try stdout.flush(); // line 34
}
// 6 construct(s) lowered to TODO markers