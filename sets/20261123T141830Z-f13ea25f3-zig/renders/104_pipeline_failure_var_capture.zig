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

    var count: []const u8 = "";
    var d: []const u8 = "";
    var files: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    d = sh2Capture(); // line 3
    files = sh2Capture(); // line 4
    try stdout.print("{s}{s}\n", .{"Files: ", files});
    try stdout.flush(); // line 5
    count = sh2Capture();
    try stdout.print("{s}{s}\n", .{"Words: ", count});
    try stdout.flush(); // line 7
    _ = sh2Run(&[_][]const u8{"cd", "/"}); // line 8
    _ = sh2Run(&[_][]const u8{"rm", "-r", "-f", d}); // line 9
    try stdout.print("{s}\n", .{"done"});
    try stdout.flush();
}