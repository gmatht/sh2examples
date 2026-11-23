const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    try stdout.print("{s}{s}\n", .{"cmdsub=", "sub-result"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"group-run"});
    try stdout.flush(); // line 5
}