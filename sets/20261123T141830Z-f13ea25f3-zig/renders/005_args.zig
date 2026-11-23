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

    var a: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    try stdout.print("{s}\n", .{"== Argument count =="});
    try stdout.flush();
    try stdout.print("{s}\n", .{sh2Env("#")});
    try stdout.flush(); // line 7
    try stdout.print("{s}\n", .{"== Arguments =="});
    try stdout.flush();
    // TODO(unsupported): listVar → sh2.listVar // line 10
    // TODO(unsupported): listVar → sh2.listVar
    {
        const sh2_items0 = [_][]const u8{sh2ListVar()};
        for (sh2_items0[0..]) |sh2_v1| {
            a = sh2_v1;
            try stdout.print("{s}{s}\n", .{"Arg: ", a});
    try stdout.flush();
        }
    }
}
// 2 construct(s) lowered to TODO markers