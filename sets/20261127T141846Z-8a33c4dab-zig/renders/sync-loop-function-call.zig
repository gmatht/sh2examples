const std = @import("std");

fn sh2ToInt(s: []const u8) i64 {
    return std.fmt.parseInt(i64, std.mem.trim(u8, s, " \t\n\r"), 10) catch 0;
}

fn sh2IntStr(n: i64) []const u8 {
    return std.fmt.allocPrint(std.heap.page_allocator, "{d}", .{n}) catch "";
}


fn f(init: std.process.Init, __args: []const []const u8) []const u8 {
    _ = init;
    return sh2IntStr(((sh2ToInt(if (__args.len >= 1) __args[1 - 1] else "")) + (sh2ToInt(if (__args.len >= 1) __args[1 - 1] else ""))));
}
pub fn main(init: std.process.Init) !void {

    var i: i64 = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    i = 0;
    while ((i < 5)) {
        try stdout.print("{s}\n", .{f(init, &[_][]const u8{ sh2IntStr(i) })});
    try stdout.flush();
        i = (i + 1);
    }
}