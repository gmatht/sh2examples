const std = @import("std");

var sh2_io: ?std.Io = null;

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

    _ = sh2Run(&[_][]const u8{"set", "-e", "-u", "-o", "pipefail"}); // line 6
    try stdout.print("{s}\n", .{"== Basic brace expansion =="});
    try stdout.flush();
    try stdout.print("{s}\n", .{"1 2 3 4 5"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"a b c"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"00 02 04"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"== Advanced brace expansion =="});
    try stdout.flush();
    try stdout.print("{s}\n", .{"a1 a2 a3 b1 b2 b3 c1 c2 c3"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"1 3 5 7 9"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"a d g j m p s v y"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"== Practical examples =="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"touch", "file_001.txt", "file_002.txt", "file_003.txt", "file_004.txt", "file_005.txt"});
    _ = sh2Run(&[_][]const u8{"ls", "\x01SH2GLOB\x01file_*.txt"}); // line 21
    _ = sh2Run(&[_][]const u8{"rm", "\x01SH2GLOB\x01file_*.txt"}); // line 22
    try stdout.print("{s}{s}\n", .{"brace_expand: ", "1 2 3 4 5"});
    try stdout.flush();
}