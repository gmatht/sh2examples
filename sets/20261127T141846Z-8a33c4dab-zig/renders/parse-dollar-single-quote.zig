const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var IFS: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    IFS = "\n\t"; // line 3
    try stdout.print("{s}", .{"IFS value (hex)="});
    try stdout.flush(); // line 4
    try stdout.print("{s}", .{"$'\\n\\t'"});
    try stdout.flush();
    try stdout.print("{s}", .{"\n"});
    try stdout.flush(); // line 4
    _ = &IFS;
}