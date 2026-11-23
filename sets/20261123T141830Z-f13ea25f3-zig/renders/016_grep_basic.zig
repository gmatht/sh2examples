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

pub fn main(init: std.process.Init) !void {

    var matched: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Exec()) or sh2Truthy(sh2Exec()));
    try stdout.print("{s}\n", .{"HELLO world"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"grep", "-i", "hello"});
    try stdout.print("{s}{s}{s}\n", .{"-e", " ", "line1\\nline2\\nline3"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"grep", "-v", "line2"});
    try stdout.print("{s}{s}{s}\n", .{"-e", " ", "first\\nsecond\\nthird"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"grep", "-n", "second"});
    try stdout.print("{s}{s}{s}\n", .{"-e", " ", "match\\nno match\\nmatch again"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"grep", "-c", "match"});
    try stdout.print("{s}\n", .{"text with pattern123 in it"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"grep", "-o", "pattern[0-9]\\+"});
    matched = sh2Capture();
    try stdout.print("{s}{s}\n", .{"  grep_exit: ", sh2Env("?")});
    try stdout.flush(); // line 25
    try stdout.print("{s}{s}\n", .{"  match_count: ", matched});
    try stdout.flush(); // line 26
}
// 4 construct(s) lowered to TODO markers