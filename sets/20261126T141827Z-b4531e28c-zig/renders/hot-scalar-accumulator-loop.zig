const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var i: i64 = 0;
    var s: i64 = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    s = 0; // line 4
    i = 0;
    while ((i < 100)) {
        s = (s + i);
        i = (i + 1);
    }
    try stdout.print("{s}{d}\n", .{"sum=", s});
    try stdout.flush(); // line 10
}