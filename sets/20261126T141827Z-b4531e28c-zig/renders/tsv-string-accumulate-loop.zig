const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var i: i64 = 0;
    var tsv: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    tsv = "";
    { // line 4
        const sh2_items0 = [_]i64{1, 2, 3, 4, 5};
        for (sh2_items0[0..]) |sh2_v1| {
            i = sh2_v1;
            tsv = std.fmt.allocPrint(std.heap.page_allocator, "{s}{d},", .{tsv, i}) catch "";
        }
    }
    try stdout.print("{s}\n", .{tsv});
    try stdout.flush(); // line 7
}