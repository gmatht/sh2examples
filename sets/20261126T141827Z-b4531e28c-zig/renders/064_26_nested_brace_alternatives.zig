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

    try stdout.print("{s}\n", .{"ax/pz ax/qz ayz"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"ad bd cd"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"src/main/a src/main/b src/test/a src/test/b docs/a docs/b"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"file.txt file.bak file.orig"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"prexpost preypost"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"source.bat source.c"});
    try stdout.flush();
    try stdout.print("{s}", .{"pre"});
    try stdout.flush(); // line 18
    try stdout.print("{s}\n", .{"xpost ypost"});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"done: ", sh2Env("?")});
    try stdout.flush(); // line 20
}