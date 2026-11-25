const std = @import("std");

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    if (true) {
        try stdout.print("{s}\n", .{"Hi!"});
    try stdout.flush();
    } else if (false) {
        try stdout.print("{s}\n", .{"Bye!"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"Other"});
    try stdout.flush();
    }
}