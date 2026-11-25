const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    if (std.mem.eql(u8, "word", "$(echo \"pattern\")")) { // line 3
        try stdout.print("{s}\n", .{"matched"});
    try stdout.flush();
    }
}