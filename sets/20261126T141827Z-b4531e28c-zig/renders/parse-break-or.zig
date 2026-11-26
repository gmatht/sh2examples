const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

var sh2_io: ?std.Io = null;
var sh2_environ: ?*std.process.Environ.Map = null;

fn sh2Env(name: []const u8) []const u8 {
    const m = sh2_environ orelse return "";
    return m.get(name) orelse "";
}

fn sh2Break() []const u8 {
    sh2TODO("break");
    return "";
}

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    while (true) {
        // TODO(unsupported): break → sh2.break
        // TODO(unsupported): break → sh2.break
        // TODO(unsupported): exec → sh2.exec
        // TODO(unsupported): exec → sh2.exec
        _ = (sh2Truthy(sh2Break()) or sh2Truthy(sh2Exec()));
    }
    try stdout.print("{s}", .{"parsed OK\n"});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"exit: ", sh2Env("?")});
    try stdout.flush(); // line 5
}
// 4 construct(s) lowered to TODO markers