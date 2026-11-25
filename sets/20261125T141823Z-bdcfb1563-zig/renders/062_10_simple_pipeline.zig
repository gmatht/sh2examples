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

pub fn main(init: std.process.Init) !void {

    var d: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    try stdout.print("{s}\n", .{"Testing simple pipeline..."});
    try stdout.flush();
    d = sh2Capture(); // line 5
    _ = sh2Run(&[_][]const u8{"mkdir", "-p", std.fmt.allocPrint(std.heap.page_allocator, "{s}/dir1", .{d}) catch "", std.fmt.allocPrint(std.heap.page_allocator, "{s}/dir2", .{d}) catch "", std.fmt.allocPrint(std.heap.page_allocator, "{s}/dir3", .{d}) catch ""}); // line 6
    _ = sh2Run(&[_][]const u8{"touch", std.fmt.allocPrint(std.heap.page_allocator, "{s}/file1", .{d}) catch "", std.fmt.allocPrint(std.heap.page_allocator, "{s}/file2", .{d}) catch ""}); // line 7
    _ = sh2Run(&[_][]const u8{"cd", d}); // line 8
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    try stdout.print("{s}{s}\n", .{"exit: ", sh2Env("?")});
    try stdout.flush(); // line 11
    _ = sh2Run(&[_][]const u8{"cd", "/"}); // line 12
    _ = sh2Run(&[_][]const u8{"rm", "-r", "-f", d}); // line 13
}
// 2 construct(s) lowered to TODO markers