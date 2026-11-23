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
    var tee_result: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    try stdout.print("{s}\n", .{"=== Output and Formatting Commands ==="});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"Echo result: ", "Hello from backticks"});
    try stdout.flush();
    d = sh2Capture(); // line 15
    // TODO(unsupported): exec → sh2.exec // line 16
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Exec()) or sh2Truthy(sh2Exec()));
    try stdout.print("{s}{s}\n", .{"Printf result: ", "Number: 42, String: test"});
    try stdout.flush();
    tee_result = sh2Capture();
    try stdout.print("{s}{s}\n", .{"Tee result: ", tee_result});
    try stdout.flush(); // line 23
    _ = sh2Run(&[_][]const u8{"rm", "-f", "test_tee.txt"}); // line 26
    try stdout.print("{s}\n", .{"=== Output and Formatting Commands Complete ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"cd", "/"}); // line 29
    _ = sh2Run(&[_][]const u8{"rm", "-r", "-f", d}); // line 30
}
// 4 construct(s) lowered to TODO markers