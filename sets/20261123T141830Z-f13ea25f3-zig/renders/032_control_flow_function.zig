const std = @import("std");


fn greet(init: std.process.Init, __args: []const []const u8) []const u8 {
    _ = init;
    return std.fmt.allocPrint(std.heap.page_allocator, "Hello, {s}!", .{(if (__args.len >= 1) __args[1 - 1] else "")}) catch "";
}
pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    try stdout.print("{s}\n", .{greet(init, &[_][]const u8{ "World" })});
    try stdout.flush();
}