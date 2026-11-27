const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var __sh2_tc0: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    __sh2_tc0 = "hello";
    if (std.mem.eql(u8, __sh2_tc0, "hello")) {
        try stdout.print("{s}\n", .{"match"});
    try stdout.flush();
    }
}