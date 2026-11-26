const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    try stdout.print("{d}\n", .{((5 + 3) & 12)});
    try stdout.flush(); // line 3
    try stdout.print("{d}\n", .{(1 << 4)});
    try stdout.flush(); // line 4
    try stdout.print("{d}\n", .{(32 >> 2)});
    try stdout.flush(); // line 5
    try stdout.print("{d}\n", .{(7 | 8)});
    try stdout.flush(); // line 6
    try stdout.print("{d}\n", .{(12 ^ 5)});
    try stdout.flush(); // line 7
    try stdout.print("{d}\n", .{(~0)});
    try stdout.flush(); // line 8
}