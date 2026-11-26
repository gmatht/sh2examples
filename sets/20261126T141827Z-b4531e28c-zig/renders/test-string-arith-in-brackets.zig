const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var a: [1024]i64 = undefined;
    var a_len: usize = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    a_len = 0; // line 3
    a[a_len] = 10;
    a_len += 1;
    a[a_len] = 20;
    a_len += 1;
    a[a_len] = 30;
    a_len += 1;
    a[a_len] = 40;
    a_len += 1;
    // TODO(unsupported): arith Index
    // TODO(unsupported): arith Index
    if ((1 < (0 + 0))) {
        try stdout.print("{s}\n", .{"arith-test-ok"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"arith-test-bad"});
    try stdout.flush();
    }
    try stdout.print("{s}\n", .{"done"});
    try stdout.flush(); // line 6
}
// 2 construct(s) lowered to TODO markers