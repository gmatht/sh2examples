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

fn sh2Run(argv: []const []const u8) []const u8 {
    const io = sh2_io orelse return "";
    const result = std.process.run(std.heap.page_allocator, io, .{
        .argv = argv,
        .stdout_limit = .limited(1 << 20),
    }) catch return "";
    return std.mem.trimEnd(u8, result.stdout, "\n");
}

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

fn sh2Pipeline() []const u8 {
    sh2TODO("pipeline");
    return "";
}


fn get_system_info(init: std.process.Init, __args: []const []const u8) []const u8 {
    _ = init;
    _ = __args;
    var info__arch__: []const u8 = "";
    var info__hostname__: []const u8 = "";
    var info__os__: []const u8 = "";
    var info__user__: []const u8 = "";
    _ = sh2Run(&[_][]const u8{"local", "-A", "info"});
    info__os__ = sh2Capture();
    info__arch__ = sh2Capture();
    info__hostname__ = sh2Capture();
    info__user__ = sh2Env("USER");
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
}
pub fn main(init: std.process.Init) !void {

    const key: []const u8 = "";
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    _ = sh2Run(&[_][]const u8{"get_system_info"}); // line 16
    _ = &key;
}
// 2 construct(s) lowered to TODO markers