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

pub fn main(init: std.process.Init) !void {

    var x: []const u8 = "";
    var z: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    x = std.fmt.allocPrint(std.heap.page_allocator, "{s}/file", .{sh2Env("HOME")}) catch ""; // line 3
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    z = std.fmt.allocPrint(std.heap.page_allocator, "{s} with {s} and {s}ext", .{x, "literal string", sh2Param()}) catch "";
    try stdout.print("{s}\n", .{z});
    try stdout.flush(); // line 6
    try stdout.print("{s}", .{"y=[literal string]\n"});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}{s}{s}", .{"x", "=[", sh2Param(), "]\n"});
    try stdout.flush();
}
// 4 construct(s) lowered to TODO markers