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

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}

fn sh2StrHasPrefix() []const u8 {
    sh2TODO("strHasPrefix");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var tmp: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    if ((!!std.mem.eql(u8, sh2Env("opt"), ""))) {
    // TODO(unsupported): strHasPrefix → sh2.strHasPrefix
    // TODO(unsupported): strHasPrefix → sh2.strHasPrefix
    } else if (sh2Truthy(sh2StrHasPrefix())) {
        // TODO(unsupported): param → sh2.param
        // TODO(unsupported): param → sh2.param
        tmp = sh2Param();
    } else {
        try stdout.print("{s}\n", .{"default"});
    try stdout.flush();
    }
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}{s}{s}", .{"tmp", "=[", sh2Param(), "]\n"});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}{s}{s}", .{"configfile", "=[", sh2Param(), "]\n"});
    try stdout.flush();
    _ = &tmp;
}
// 8 construct(s) lowered to TODO markers