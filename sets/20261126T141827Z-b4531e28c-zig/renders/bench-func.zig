const std = @import("std");


fn f(init: std.process.Init, __args: []const []const u8) []const u8 {
    _ = init;
    _ = __args;
    return "f-called";
}
pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    try stdout.print("{s}\n", .{f(init, &[_][]const u8{  })});
    try stdout.flush();
    try stdout.print("{s}\n", .{f(init, &[_][]const u8{  })});
    try stdout.flush();
}