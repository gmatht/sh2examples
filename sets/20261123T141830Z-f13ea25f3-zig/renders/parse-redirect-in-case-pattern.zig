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

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}


fn _docker_to_extglob(init: std.process.Init, __args: []const []const u8) []const u8 {
    _ = init;
    return (if (__args.len >= 1) __args[1 - 1] else "");
}
pub fn main(init: std.process.Init) !void {

    var __sh2_casepat_0: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    __sh2_casepat_0 = _docker_to_extglob(init, &[_][]const u8{ sh2Env("subcommands") });
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    if (std.mem.eql(u8, sh2Param(), "$__sh2_casepat_0")) {
        try stdout.print("{s}\n", .{"match"});
    try stdout.flush();
    }
    _ = &__sh2_casepat_0;
}
// 2 construct(s) lowered to TODO markers