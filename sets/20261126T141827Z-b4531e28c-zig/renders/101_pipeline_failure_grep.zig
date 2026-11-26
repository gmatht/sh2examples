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

    try stdout.print("{s}\n", .{"Grep test:"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"alpha beta gamma"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"grep", "beta"});
    try stdout.print("{s}\n", .{"---"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"alpha beta gamma"});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"grep", "-o", "beta"});
    try stdout.print("{s}\n", .{"done"});
    try stdout.flush();
}