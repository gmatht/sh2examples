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

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

fn sh2Redirect() []const u8 {
    sh2TODO("redirect");
    return "";
}

pub fn main(init: std.process.Init) !void {
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    if ((!!std.mem.eql(u8, sh2Env("1"), "test"))) {
        // TODO(unsupported): redirect → sh2.redirect
        // TODO(unsupported): redirect → sh2.redirect
        // TODO(unsupported): exec → sh2.exec
        // TODO(unsupported): exec → sh2.exec
        _ = (sh2Truthy(sh2Redirect()) and sh2Truthy(sh2Exec()));
    }
}
// 4 construct(s) lowered to TODO markers