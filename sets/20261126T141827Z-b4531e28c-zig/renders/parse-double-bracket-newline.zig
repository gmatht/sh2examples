const std = @import("std");

pub fn main(init: std.process.Init) !void {

    const other: []const u8 = "";
    const var_: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    if ((!std.mem.eql(u8, var_, "") and (!!std.mem.eql(u8, other, "")))) {
        try stdout.print("{s}\n", .{"test"});
    try stdout.flush();
    }
}