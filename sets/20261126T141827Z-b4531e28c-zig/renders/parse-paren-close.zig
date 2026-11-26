const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var a: i64 = 0;
    var b: i64 = 0;
    var c: i64 = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    c = 13;
    try stdout.print("{s}{d}{s}", .{"a=[", 9, "]\n"});
    try stdout.flush();
    try stdout.print("{s}{d}{s}", .{"b=[", 4, "]\n"});
    try stdout.flush(); // line 6
    try stdout.print("{s}{d}{s}", .{"c=[", c, "]\n"});
    try stdout.flush(); // line 7
    try stdout.print("{d}\n", .{(13 + c)});
    try stdout.flush(); // line 8
    _ = &a;
    _ = &b;
}