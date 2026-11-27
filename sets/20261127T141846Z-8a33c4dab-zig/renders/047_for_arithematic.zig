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

    j = "1"; // line 1
    {
        const sh2_items0 = [_][]const u8{"1", "2", "3", "4", "5"};
        for (sh2_items0[0..]) |sh2_v1| {
            i = sh2_v1;
            j = sh2IntStr(((sh2ToInt(j)) * (sh2ToInt(i))));
        }
    }
    try stdout.print("{s}\n", .{sh2IntStr(j)});
    try stdout.flush(); // line 6
}