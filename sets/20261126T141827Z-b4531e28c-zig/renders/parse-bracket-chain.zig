const std = @import("std");

pub fn main(init: std.process.Init) !void {

    const VAR1: []const u8 = "";
    const VAR2: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    if (((!!std.mem.eql(u8, VAR1, "")) and (!!std.mem.eql(u8, VAR2, "")))) {
        try stdout.print("{s}\n", .{"Both empty"});
    try stdout.flush();
    }
}