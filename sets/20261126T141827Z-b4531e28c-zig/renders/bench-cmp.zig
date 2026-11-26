const std = @import("std");

fn sh2B2S(b: bool) []const u8 {
    return if (b) "1" else "0";
}

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

    _ = sh2B2S(std.mem.eql(u8, "abc", "abc")); // line 3
    try stdout.print("{s}{s}\n", .{"eq=", sh2Env("?")});
    try stdout.flush(); // line 3
    _ = sh2B2S(!std.mem.eql(u8, "abc", "xyz")); // line 4
    try stdout.print("{s}{s}\n", .{"ne=", sh2Env("?")});
    try stdout.flush(); // line 4
    _ = sh2B2S((5 < 10)); // line 5
    try stdout.print("{s}{s}\n", .{"lt=", sh2Env("?")});
    try stdout.flush(); // line 5
}