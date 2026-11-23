const std = @import("std");

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

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    _ = sh2Run(&[_][]const u8{"set", "-e", "-u", "-o", "pipefail"}); // line 4
    try stdout.print("{s}\n", .{"== Practical examples =="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"touch", "file_001.txt", "file_002.txt", "file_003.txt", "file_004.txt", "file_005.txt"});
    _ = sh2Run(&[_][]const u8{"ls", "\x01SH2GLOB\x01file_*.txt"}); // line 9
    _ = sh2Run(&[_][]const u8{"rm", "\x01SH2GLOB\x01file_*.txt"}); // line 10
    try stdout.print("{s}{s}\n", .{"done: ", sh2Env("?")});
    try stdout.flush(); // line 12
}