const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    if (("5\"\\>\"10" .len != 0)) {
        try stdout.print("{s}\n", .{"a > b"});
    try stdout.flush();
    }
    try stdout.print("{s}", .{"a=[5]\n"});
    try stdout.flush();
    try stdout.print("{s}", .{"b=[10]\n"});
    try stdout.flush();
}