const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
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

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var d: []const u8 = "";
    sh2_io = init.io;

    d = sh2Capture(); // line 4
    _ = sh2Run(&[_][]const u8{"touch", std.fmt.allocPrint(std.heap.page_allocator, "{s}/a", .{d}) catch "", std.fmt.allocPrint(std.heap.page_allocator, "{s}/b", .{d}) catch ""}); // line 5
    _ = sh2Run(&[_][]const u8{"ln", std.fmt.allocPrint(std.heap.page_allocator, "{s}/a", .{d}) catch "", std.fmt.allocPrint(std.heap.page_allocator, "{s}/a2", .{d}) catch ""}); // line 6
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Exec()) and sh2Truthy(sh2Exec()));
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Exec()) or sh2Truthy(sh2Exec()));
    _ = sh2Run(&[_][]const u8{"rm", "-r", "-f", d}); // line 9
}
// 8 construct(s) lowered to TODO markers