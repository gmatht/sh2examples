const std = @import("std");

fn sh2IntStr(n: i64) []const u8 {
    return std.fmt.allocPrint(std.heap.page_allocator, "{d}", .{n}) catch "";
}

pub fn main(init: std.process.Init) !void {

    var a: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    a = "1"; // line 1
    try stdout.print("{s}\n", .{sh2IntStr(a)});
    try stdout.flush(); // line 2
    a = "2"; // line 3
    try stdout.print("{s}\n", .{sh2IntStr(a)});
    try stdout.flush();
    try stdout.print("{s}\n", .{sh2IntStr(a)});
    try stdout.flush(); // line 4
    try stdout.print("{s}\n", .{sh2IntStr(a)});
    try stdout.flush(); // line 5
}