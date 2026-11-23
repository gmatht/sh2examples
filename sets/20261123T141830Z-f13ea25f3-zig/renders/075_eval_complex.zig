const std = @import("std");

var sh2_io: ?std.Io = null;

fn sh2Run(argv: []const []const u8) []const u8 {
    const io = sh2_io orelse return "";
    const result = std.process.run(std.heap.page_allocator, io, .{
        .argv = argv,
        .stdout_limit = .limited(1 << 20),
    }) catch return "";
    return std.mem.trimEnd(u8, result.stdout, "\n");
}

pub fn main(init: std.process.Init) !void {

    var x: i64 = 0;
    sh2_io = init.io;

    x = 42; // line 4
    _ = sh2Run(&[_][]const u8{"eval", std.fmt.allocPrint(std.heap.page_allocator, "echo \"The answer is {d}\"", .{x}) catch ""}); // line 5
}