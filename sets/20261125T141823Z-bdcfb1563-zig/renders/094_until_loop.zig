const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var count: i64 = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    count = 3;
    while ((!(count == 0))) {
        try stdout.print("{s}{d}\n", .{"count=", count});
    try stdout.flush();
        count = (count - 1);
    }
}