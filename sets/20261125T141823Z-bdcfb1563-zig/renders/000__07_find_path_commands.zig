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

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var d: []const u8 = "";
    var found_files: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    d = sh2Capture(); // line 6
    _ = sh2Run(&[_][]const u8{"mkdir", "-p", std.fmt.allocPrint(std.heap.page_allocator, "{s}/sub", .{d}) catch ""}); // line 7
    _ = sh2Run(&[_][]const u8{"touch", std.fmt.allocPrint(std.heap.page_allocator, "{s}/a.sh", .{d}) catch "", std.fmt.allocPrint(std.heap.page_allocator, "{s}/b.sh", .{d}) catch "", std.fmt.allocPrint(std.heap.page_allocator, "{s}/sub/c.sh", .{d}) catch "", std.fmt.allocPrint(std.heap.page_allocator, "{s}/data.txt", .{d}) catch ""}); // line 8
    found_files = sh2Capture();
    try stdout.print("{s}\n", .{"Found shell scripts:"});
    try stdout.flush();
    try stdout.print("{s}\n", .{found_files});
    try stdout.flush(); // line 11
    _ = sh2Run(&[_][]const u8{"rm", "-r", "-f", d}); // line 12
}