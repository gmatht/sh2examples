const std = @import("std");

var sh2_io: ?std.Io = null;
var sh2_environ: ?*std.process.Environ.Map = null;

fn sh2Env(name: []const u8) []const u8 {
    const m = sh2_environ orelse return "";
    return m.get(name) orelse "";
}

pub fn main(init: std.process.Init) !void {

    var i: i64 = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    i = 0; // line 5
    i += 1;
    try stdout.print("{s}{s}\n", .{"post0=", sh2Env("?")});
    try stdout.flush(); // line 7
    i += 1;
    try stdout.print("{s}{s}\n", .{"pre=", sh2Env("?")});
    try stdout.flush(); // line 9
    i = 5; // line 10
    i += 1;
    try stdout.print("{s}{s}\n", .{"post5=", sh2Env("?")});
    try stdout.flush(); // line 12
}