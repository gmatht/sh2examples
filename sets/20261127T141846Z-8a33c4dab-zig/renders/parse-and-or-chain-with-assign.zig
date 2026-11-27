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

fn sh2Assign() []const u8 {
    sh2TODO("assign");
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

    // TODO(unsupported): exec → sh2.exec // line 2
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): assign → sh2.assign
    // TODO(unsupported): assign → sh2.assign
    // TODO(unsupported): assign → sh2.assign
    // TODO(unsupported): assign → sh2.assign
    _ = ((sh2Truthy(sh2Exec()) and sh2Truthy(sh2Assign())) or sh2Truthy(sh2Assign()));
    try stdout.print("{s}\n", .{sh2Env("result")});
    try stdout.flush(); // line 5
}
// 6 construct(s) lowered to TODO markers