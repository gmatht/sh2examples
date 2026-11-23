const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var i: i64 = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    i = 0;
    while ((i < 3)) {
        try stdout.print("{s}{d}\n", .{"i=", i});
    try stdout.flush();
        i = (i + 1);
    }
}