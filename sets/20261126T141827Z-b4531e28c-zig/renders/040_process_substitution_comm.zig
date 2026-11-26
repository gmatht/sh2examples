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

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var __ps_tmp0: []const u8 = "";
    var __ps_tmp1: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    _ = sh2Run(&[_][]const u8{"set", "-e", "-u", "-o", "pipefail"}); // line 4
    try stdout.print("{s}\n", .{"== Process substitution with comm =="});
    try stdout.flush();
    __ps_tmp0 = sh2Capture();
    try std.fs.cwd().writeFile(.{ .sub_path = __ps_tmp0, .data = "a\nb\n" });
    __ps_tmp1 = sh2Capture();
    try std.fs.cwd().writeFile(.{ .sub_path = __ps_tmp1, .data = "b\nc\n" });
    _ = sh2Run(&[_][]const u8{"comm", "-12", __ps_tmp0, __ps_tmp1});
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = sh2Exec();
    try stdout.print("{s}{s}\n", .{"exit: ", sh2Env("?")});
    try stdout.flush(); // line 9
}
// 2 construct(s) lowered to TODO markers