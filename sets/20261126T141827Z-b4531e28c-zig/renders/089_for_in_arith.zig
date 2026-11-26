const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var i: i64 = 0;
    var total: i64 = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    total = 0; // line 4
    { // line 5
        const sh2_items0 = [_]i64{1, 2, 3, 4, 5};
        for (sh2_items0[0..]) |sh2_v1| {
            i = sh2_v1;
            total = (total + i);
        }
    }
    try stdout.print("{s}{d}\n", .{"total=", total});
    try stdout.flush(); // line 8
}