const std = @import("std");

fn sh2IntStr(n: i64) []const u8 {
    return std.fmt.allocPrint(std.heap.page_allocator, "{d}", .{n}) catch "";
}

pub fn main(init: std.process.Init) !void {

    var i: i64 = 0;
    var j: i64 = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    i = 0; // line 3
    i = ((i) + (1)); // line 4
    j = ((j) + (1)); // line 5
    try stdout.print("{s}{s}{s}\n", .{sh2IntStr(i), " ", sh2IntStr(j)});
    try stdout.flush(); // line 6
}