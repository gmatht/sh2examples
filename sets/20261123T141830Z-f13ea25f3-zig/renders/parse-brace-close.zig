const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var i: i64 = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    try stdout.print("{s}\n", .{"1 2 3 4 5"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"file.txt file.bak"});
    try stdout.flush();
    {
        const sh2_items0 = [_]i64{1, 2, 3};
        for (sh2_items0[0..]) |sh2_v1| {
            i = sh2_v1;
            try stdout.print("{d}\n", .{i});
    try stdout.flush();
        }
    }
}