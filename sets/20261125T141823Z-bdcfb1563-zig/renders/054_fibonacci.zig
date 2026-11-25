const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var a: i64 = 0;
    var b: i64 = 0;
    var i: i64 = 0;
    var temp: i64 = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    try stdout.print("{s}\n", .{"=== Fibonacci Sequence (first 20 numbers) ==="});
    try stdout.flush();
    a = 0; // line 9
    b = 1; // line 10
    try stdout.print("{s}\n", .{"Fibonacci numbers:"});
    try stdout.flush();
    try stdout.print("{d}{s}{d}{s}", .{a, " ", b, " "});
    try stdout.flush(); // line 13
    {
        const sh2_items0 = [_]i64{3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20};
        for (sh2_items0[0..]) |sh2_v1| {
            i = sh2_v1;
            temp = (a + b);
            try stdout.print("{d}{s}", .{temp, " "});
    try stdout.flush();
            a = b;
            b = temp;
        }
    }
    try stdout.print("{s}\n", .{""});
    try stdout.flush();
    try stdout.print("{s}\n", .{"Done!"});
    try stdout.flush();
    _ = &i;
}