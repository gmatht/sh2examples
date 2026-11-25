const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    if (std.mem.eql(u8, "hello", "hello")) {
        try stdout.print("{s}\n", .{"greeting"});
    try stdout.flush();
    } else if (std.mem.eql(u8, "hello", "bye")) {
        try stdout.print("{s}\n", .{"farewell"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"unknown"});
    try stdout.flush();
    }
}