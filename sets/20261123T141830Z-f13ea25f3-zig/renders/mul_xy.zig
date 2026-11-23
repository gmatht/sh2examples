const std = @import("std");

fn sh2ToInt(s: []const u8) i64 {
    return std.fmt.parseInt(i64, std.mem.trim(u8, s, " \t\n\r"), 10) catch 0;
}

fn sh2IntStr(n: i64) []const u8 {
    return std.fmt.allocPrint(std.heap.page_allocator, "{d}", .{n}) catch "";
}


fn mul_xy(init: std.process.Init, __args: []const []const u8) []const u8 {
    _ = init;
    var x: []const u8 = "";
    var y: []const u8 = "";
    x = (if (__args.len >= 1) __args[1 - 1] else "");
    y = (if (__args.len >= 2) __args[2 - 1] else "");
    return sh2IntStr((sh2ToInt(x) * sh2ToInt(y)));
}
pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    try stdout.print("{s}\n", .{mul_xy(init, &[_][]const u8{ "3", "4" })});
    try stdout.flush();
}