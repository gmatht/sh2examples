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

    var x: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    x = "1"; // line 6
    x = "2"; // line 7
    _ = sh2Run(&[_][]const u8{"wait"}); // line 8
    try stdout.print("{s}{s}\n", .{"x=", x});
    try stdout.flush(); // line 9
}