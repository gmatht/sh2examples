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

    {
        const sh2_items0 = [_]i64{1, 2, 3, 4, 5};
        for (sh2_items0[0..]) |sh2_v1| {
            i = sh2_v1;
            try stdout.print("{s}{d}\n", .{"Number: ", i});
    try stdout.flush();
        }
    }
    {
        const sh2_items2 = [_]i64{1, 2, 3};
        for (sh2_items2[0..]) |sh2_v3| {
            i = sh2_v3;
            j = (j + 1);
        }
    }
    try stdout.print("{s}\n", .{sh2IntStr(j)});
    try stdout.flush(); // line 8
    i = 0;
    while ((i < 10)) {
        try stdout.print("{s}{d}\n", .{"Counter: ", i});
    try stdout.flush();
        i = (i + 1);
    }
}