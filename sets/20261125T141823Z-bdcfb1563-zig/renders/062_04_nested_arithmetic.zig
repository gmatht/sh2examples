const std = @import("std");

pub fn main(init: std.process.Init) !void {

    const result: i64 = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    try stdout.print("{s}\n", .{"Testing nested arithmetic..."});
    try stdout.flush();
    try stdout.print("{s}{d}\n", .{"Complex arithmetic: ", 40});
    try stdout.flush();
    _ = &result;
}