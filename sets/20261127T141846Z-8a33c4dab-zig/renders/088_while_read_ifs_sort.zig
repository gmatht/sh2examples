const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

var sh2_io: ?std.Io = null;

fn sh2Run(argv: []const []const u8) []const u8 {
    const io = sh2_io orelse return "";
    const result = std.process.run(std.heap.page_allocator, io, .{
        .argv = argv,
        .stdout_limit = .limited(1 << 20),
    }) catch return "";
    return std.mem.trimEnd(u8, result.stdout, "\n");
}

fn sh2Pipeline() []const u8 {
    sh2TODO("pipeline");
    return "";
}

fn sh2WriteFile() []const u8 {
    sh2TODO("writeFile");
    return "";
}

pub fn main(init: std.process.Init) !void {
    sh2_io = init.io;

    try std.fs.cwd().writeFile(.{ .sub_path = "/tmp/088_data.txt", .data = "b:2\n" });
    // TODO(unsupported): WriteFile append → sh2.writeFile
    // TODO(unsupported): WriteFile append → sh2.writeFile
    // TODO(unsupported): pipeline → sh2.pipeline // line 7
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    _ = sh2Run(&[_][]const u8{"rm", "-f", "/tmp/088_data.txt"}); // line 10
}
// 4 construct(s) lowered to TODO markers