const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var y: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    y = std.fmt.allocPrint(std.heap.page_allocator, "{s}world", .{"hello"}) catch "";
    try stdout.print("{s}\n", .{y});
    try stdout.flush(); // line 5
    try stdout.print("{s}", .{"x=[hello]\n"});
    try stdout.flush();
}