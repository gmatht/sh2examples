const std = @import("std");

fn sh2ToInt(s: []const u8) i64 {
    return std.fmt.parseInt(i64, std.mem.trim(u8, s, " \t\n\r"), 10) catch 0;
}

pub fn main(init: std.process.Init) !void {

    var __sh2_tc0: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    __sh2_tc0 = "foo";
    if ((sh2ToInt(__sh2_tc0) != 0)) {
        try stdout.print("{s}\n", .{"bar"});
    try stdout.flush();
    }
}