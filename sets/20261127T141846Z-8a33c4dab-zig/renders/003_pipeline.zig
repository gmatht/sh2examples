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

fn sh2Pipeline() []const u8 {
    sh2TODO("pipeline");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var d: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    d = sh2Capture(); // line 4
    // TODO(unsupported): exec → sh2.exec // line 5
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Exec()) or sh2Truthy(sh2Exec()));
    try std.fs.cwd().writeFile(.{ .sub_path = "file.txt", .data = "apple\napple\n" }); // line 6
    try std.fs.cwd().writeFile(.{ .sub_path = "note.txt", .data = "hello.txt\n" }); // line 7
    try std.fs.cwd().writeFile(.{ .sub_path = "s.sh", .data = "function f() { echo hi; }\n" }); // line 8
    _ = sh2Run(&[_][]const u8{"ls"});
    _ = sh2Run(&[_][]const u8{"grep", "\\.txt$"});
    _ = sh2Run(&[_][]const u8{"wc", "-l"});
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 10
    // TODO(unsupported): pipeline → sh2.pipeline // line 11
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 12
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 14
    _ = sh2Run(&[_][]const u8{"cat", "file.txt"});
    _ = sh2Run(&[_][]const u8{"tr", "a", "b"});
    _ = sh2Run(&[_][]const u8{"grep", "hello"});
    try stdout.print("{s}\n", .{""});
    try stdout.flush(); // line 16
    // TODO(unsupported): pipeline → sh2.pipeline // line 17
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    _ = sh2Run(&[_][]const u8{"cd", "/"}); // line 18
    _ = sh2Run(&[_][]const u8{"rm", "-r", "-f", d}); // line 19
}
// 10 construct(s) lowered to TODO markers