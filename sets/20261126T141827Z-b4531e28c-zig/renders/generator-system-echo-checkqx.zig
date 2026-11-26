const std = @import("std");

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


fn INFO(init: std.process.Init, __args: []const []const u8) []const u8 {
    _ = init;
    _ = __args;
    _ = sh2Run(&[_][]const u8{"/bin/echo", "-e", std.fmt.allocPrint(std.heap.page_allocator, "hello {s}", .{sh2Env("@")}) catch ""});
}
pub fn main(init: std.process.Init) !void {
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    _ = sh2Run(&[_][]const u8{"INFO", "test"});
}