const std = @import("std");

var sh2_io: ?std.Io = null;
var sh2_environ: ?*std.process.Environ.Map = null;

fn sh2Env(name: []const u8) []const u8 {
    const m = sh2_environ orelse return "";
    return m.get(name) orelse "";
}

pub fn main(init: std.process.Init) !void {

    const hex_val: i64 = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    try stdout.print("{s}\n", .{"0x1234"});
    try stdout.flush(); // line 2
    try stdout.print("{s}{d}\n", .{"hex_val: ", 255});
    try stdout.flush(); // line 3
    try stdout.print("{s}{s}\n", .{"done: ", sh2Env("?")});
    try stdout.flush(); // line 4
    _ = &hex_val;
}