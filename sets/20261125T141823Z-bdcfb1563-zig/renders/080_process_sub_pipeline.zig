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

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    try std.fs.cwd().writeFile(.{ .sub_path = "/tmp/paste_a.txt", .data = "a\n" }); // line 3
    try std.fs.cwd().writeFile(.{ .sub_path = "/tmp/paste_b.txt", .data = "b\n" }); // line 4
    _ = sh2Run(&[_][]const u8{"paste", "/tmp/paste_a.txt", "/tmp/paste_b.txt"}); // line 5
    _ = sh2Run(&[_][]const u8{"rm", "-f", "/tmp/paste_a.txt", "/tmp/paste_b.txt"}); // line 6
    try stdout.print("{s}{s}\n", .{"exit: ", sh2Env("?")});
    try stdout.flush(); // line 8
}