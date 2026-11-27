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

    var matched: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    try stdout.print("{s}{s}{s}\n", .{"-e", " ", "match1\\nmatch2\\nmatch3\\nmatch4"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"grep", "-m", "2", "match"});
    try stdout.print("{s}\n", .{"text with pattern in it"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"grep", "-b", "pattern"});
    try std.fs.cwd().writeFile(.{ .sub_path = "temp_file.txt", .data = "content\n" });
    _ = sh2Run(&[_][]const u8{"grep", "-h", "content", "temp_file.txt"});
    _ = sh2Run(&[_][]const u8{"grep", "-H", "content", "temp_file.txt"});
    _ = sh2Run(&[_][]const u8{"grep", "-Z", "-l", "pattern", "temp_file.txt"});
    _ = sh2Run(&[_][]const u8{"tr", "\\0", "\\n"});
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Pipeline()) or sh2Truthy(sh2Exec()));
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = ((sh2Truthy(sh2Exec()) and sh2Truthy(sh2Exec())) or sh2Truthy(sh2Exec()));
    _ = sh2Run(&[_][]const u8{"rm", "temp_file.txt"}); // line 29
    matched = sh2Capture();
    try stdout.print("{s}{s}\n", .{"  grep_exit: ", sh2Env("?")});
    try stdout.flush(); // line 32
    try stdout.print("{s}{s}\n", .{"  match_count: ", matched});
    try stdout.flush(); // line 33
}
// 10 construct(s) lowered to TODO markers