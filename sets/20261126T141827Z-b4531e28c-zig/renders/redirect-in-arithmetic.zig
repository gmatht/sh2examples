const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var arr: [1024]i64 = undefined;
    var arr_len: usize = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    arr_len = 0; // line 4
    arr[arr_len] = 10;
    arr_len += 1;
    arr[arr_len] = 20;
    arr_len += 1;
    arr[arr_len] = 30;
    arr_len += 1;
    // TODO(unsupported): arith Index // line 5
    try stdout.print("{d}\n", .{@intFromBool(0 > 2)});
    try stdout.flush();
}
// 1 construct(s) lowered to TODO markers