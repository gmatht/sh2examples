const std = @import("std");

var sh2_io: ?std.Io = null;
var sh2_environ: ?*std.process.Environ.Map = null;

fn sh2Env(name: []const u8) []const u8 {
    const m = sh2_environ orelse return "";
    return m.get(name) orelse "";
}

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    try stdout.print("{s}\n", .{"Testing simple case patterns..."});
    try stdout.flush();
    if ((!!std.mem.eql(u8, sh2Env("1"), "test"))) {
        try stdout.print("{s}\n", .{"Matched test"});
    try stdout.flush();
    } else {
        try stdout.print("{s}\n", .{"Default case"});
    try stdout.flush();
    }
}