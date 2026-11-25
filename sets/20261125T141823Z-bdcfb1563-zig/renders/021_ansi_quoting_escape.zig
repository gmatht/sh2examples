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

    _ = sh2Run(&[_][]const u8{"set", "-e", "-u", "-o", "pipefail"}); // line 4
    try stdout.print("{s}\n", .{"== Escape sequences =="});
    try stdout.flush();
    try stdout.print("{s}\n", .{"bell\x07"});
    try stdout.flush(); // line 7
    try stdout.print("{s}\n", .{"backspace\x08"});
    try stdout.flush(); // line 8
    try stdout.print("{s}\n", .{"formfeed\x0c"});
    try stdout.flush(); // line 9
    try stdout.print("{s}\n", .{"newline\n"});
    try stdout.flush(); // line 10
    try stdout.print("{s}\n", .{"carriage\rreturn"});
    try stdout.flush(); // line 11
    try stdout.print("{s}\n", .{"tab\tseparated"});
    try stdout.flush(); // line 12
    try stdout.print("{s}\n", .{"vertical\x0btab"});
    try stdout.flush(); // line 13
}