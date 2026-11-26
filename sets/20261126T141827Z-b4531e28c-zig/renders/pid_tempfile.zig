const std = @import("std");

var sh2_io: ?std.Io = null;
var sh2_environ: ?*std.process.Environ.Map = null;

fn sh2Env(name: []const u8) []const u8 {
    const m = sh2_environ orelse return "";
    return m.get(name) orelse "";
}

fn sh2WriteFile(p: []const u8, data: []const u8, append: bool) void {
    const f = std.fs.cwd().createFile(p, .{ .truncate = !append }) catch return;
    defer f.close();
    f.writeAll(data) catch {};
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

    var tmpf: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;
    sh2_environ = init.environ_map;

    tmpf = std.fmt.allocPrint(std.heap.page_allocator, "/tmp/{s}.txt", .{sh2Env("$")}) catch ""; // line 3
    try stdout.print("{s}\n", .{"hello"});
    try stdout.flush();
    sh2WriteFile(tmpf, "", false);
    _ = sh2Run(&[_][]const u8{"cat", tmpf}); // line 5
    _ = sh2Run(&[_][]const u8{"rm", tmpf}); // line 6
}