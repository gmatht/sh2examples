const std = @import("std");

fn sh2ToInt(s: []const u8) i64 {
    return std.fmt.parseInt(i64, std.mem.trim(u8, s, " \t\n\r"), 10) catch 0;
}

fn sh2IntStr(n: i64) []const u8 {
    return std.fmt.allocPrint(std.heap.page_allocator, "{d}", .{n}) catch "";
}

pub fn main(init: std.process.Init) !void {

    var i: []const u8 = "";
    var j: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    i = sh2IntStr(2);
    j = sh2IntStr(((sh2ToInt(i)) + (sh2ToInt(i)))); // line 5
    try stdout.print("{s}{s}{s}{s}\n", .{"i=", i, ", j=", j});
    try stdout.flush(); // line 6
}