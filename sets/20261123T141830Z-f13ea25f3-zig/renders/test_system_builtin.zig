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
    var result1: []const u8 = "";
    var result2: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    try stdout.print("{s}\n", .{"Testing system calls with builtin commands"});
    try stdout.flush();
    d = sh2Capture(); // line 4
    // TODO(unsupported): exec → sh2.exec // line 5
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Exec()) or sh2Truthy(sh2Exec()));
    try std.fs.cwd().writeFile(.{ .sub_path = "a.txt", .data = "a\n" }); // line 6
    try std.fs.cwd().writeFile(.{ .sub_path = "b.txt", .data = "b\n" }); // line 7
    _ = sh2Run(&[_][]const u8{"mkdir", "sub"}); // line 8
    try std.fs.cwd().writeFile(.{ .sub_path = "sub/c.txt", .data = "c\n" }); // line 9
    result1 = sh2Capture(); // line 11
    result2 = sh2Capture();
    try stdout.print("{s}\n", .{"Results:"});
    try stdout.flush();
    try stdout.print("{s}\n", .{result1});
    try stdout.flush(); // line 15
    try stdout.print("{s}\n", .{result2});
    try stdout.flush(); // line 16
    _ = sh2Run(&[_][]const u8{"cd", "/"}); // line 17
    _ = sh2Run(&[_][]const u8{"rm", "-r", "-f", d}); // line 18
}
// 4 construct(s) lowered to TODO markers