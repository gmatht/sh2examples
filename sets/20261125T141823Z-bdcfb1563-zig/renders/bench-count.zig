const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var __n: i64 = 0;
    var i: i64 = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    i = 1; // line 3
    __n = 0;
    while ((__n < 1000)) {
        i = (i + 1);
        __n = (__n + 1);
    }
    try stdout.print("{s}{d}\n", .{"count=", i});
    try stdout.flush(); // line 9
}