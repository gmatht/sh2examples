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


fn complex_function(init: std.process.Init, __args: []const []const u8) []const u8 {
    _ = init;
    _ = __args;
    return std.fmt.allocPrint(std.heap.page_allocator, "fn args: {s}", .{sh2Env("*")}) catch "";
}
pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{complex_function(init, &[_][]const u8{ "--long-option=value with spaces", "--array-option", "item1", "item2", "item3", "--flag", "positional argument", sh2Param(), "computed" })});
    try stdout.flush();
}
// 2 construct(s) lowered to TODO markers