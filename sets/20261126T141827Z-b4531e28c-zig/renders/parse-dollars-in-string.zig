const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

var sh2_io: ?std.Io = null;
var sh2_environ: ?*std.process.Environ.Map = null;

fn sh2Env(name: []const u8) []const u8 {
    const m = sh2_environ orelse return "";
    return m.get(name) orelse "";
}

fn sh2ListVar() []const u8 {
    sh2TODO("listVar");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var i: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    // TODO(unsupported): listVar → sh2.listVar // line 2
    // TODO(unsupported): listVar → sh2.listVar
    {
        const sh2_items0 = [_][]const u8{sh2ListVar()};
        for (sh2_items0[0..]) |sh2_v1| {
            i = sh2_v1;
            try stdout.print("{s}\n", .{i});
    try stdout.flush();
        }
    }
    { // line 3
        const sh2_items2 = [_][]const u8{sh2Env("*")};
        for (sh2_items2[0..]) |sh2_v3| {
            i = sh2_v3;
            try stdout.print("{s}\n", .{i});
    try stdout.flush();
        }
    }
}
// 2 construct(s) lowered to TODO markers