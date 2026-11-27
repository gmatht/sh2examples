const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var lang: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    { // line 2
        const sh2_items0 = [_][]const u8{"cs", "de", "es", "fi", "fr"};
        for (sh2_items0[0..]) |sh2_v1| {
            lang = sh2_v1;
            try stdout.print("{s}\n", .{lang});
    try stdout.flush();
        }
    }
}