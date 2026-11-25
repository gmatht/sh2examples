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

fn sh2Redirect() []const u8 {
    sh2TODO("redirect");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    try stdout.print("{s}\n", .{"start"});
    try stdout.flush();
    // TODO(unsupported): redirect → sh2.redirect
    // TODO(unsupported): redirect → sh2.redirect
    _ = (sh2Truthy(sh2Redirect()) or true);
    try stdout.print("{s}\n", .{"end"});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"done: ", sh2Env("?")});
    try stdout.flush(); // line 9
}
// 2 construct(s) lowered to TODO markers