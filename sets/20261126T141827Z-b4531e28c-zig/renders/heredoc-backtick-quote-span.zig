const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

var sh2_io: ?std.Io = null;

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

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    _ = sh2Run(&[_][]const u8{"cat"});
    sh2WriteFile("x.py", "", false);
    // TODO(unsupported): capture → sh2.capture
    // TODO(unsupported): capture → sh2.capture
    try stdout.print("{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}{s}", .{"w", "r", "o", "t", "e", " ", sh2Capture(), " ", "b", "y", "t", "e", "s", " ", "t", "o", " ", "x.py", "\\", "n"});
    try stdout.flush();
}
// 2 construct(s) lowered to TODO markers