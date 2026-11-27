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
    var sha256_result: []const u8 = "";
    var sha512_result: []const u8 = "";
    var strings_result: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    try stdout.print("{s}\n", .{"=== Checksum Commands ==="});
    try stdout.flush();
    d = sh2Capture(); // line 10
    // TODO(unsupported): exec → sh2.exec // line 11
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Exec()) or sh2Truthy(sh2Exec()));
    try std.fs.cwd().writeFile(.{ .sub_path = "test_checksum.txt", .data = "test content\n" });
    sha256_result = sh2Capture(); // line 13
    try stdout.print("{s}{s}\n", .{"SHA256 result: ", sha256_result});
    try stdout.flush(); // line 14
    sha512_result = sh2Capture(); // line 18
    try stdout.print("{s}{s}\n", .{"SHA512 result: ", sha512_result});
    try stdout.flush(); // line 19
    strings_result = sh2Capture(); // line 23
    try stdout.print("{s}\n", .{"Strings result:"});
    try stdout.flush();
    try stdout.print("{s}\n", .{strings_result});
    try stdout.flush(); // line 25
    _ = sh2Run(&[_][]const u8{"rm", "-f", "test_checksum.txt"}); // line 28
    try stdout.print("{s}\n", .{"=== Checksum Commands Complete ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"cd", "/"}); // line 31
    _ = sh2Run(&[_][]const u8{"rm", "-r", "-f", d}); // line 32
}
// 4 construct(s) lowered to TODO markers