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

fn sh2Redirect() []const u8 {
    sh2TODO("redirect");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var cp_result: []const u8 = "";
    var d: []const u8 = "";
    var mkdir_result: []const u8 = "";
    var mv_result: []const u8 = "";
    var rm_result: []const u8 = "";
    var touch_result: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    try stdout.print("{s}\n", .{"=== File Manipulation Commands ==="});
    try stdout.flush();
    d = sh2Capture(); // line 10
    // TODO(unsupported): exec → sh2.exec // line 11
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Exec()) or sh2Truthy(sh2Exec()));
    try std.fs.cwd().writeFile(.{ .sub_path = "test_file.txt", .data = "test content\n" });
    cp_result = sh2Capture();
    try stdout.print("{s}{s}\n", .{"Copy result: ", cp_result});
    try stdout.flush(); // line 14
    // TODO(unsupported): redirect → sh2.redirect
    // TODO(unsupported): redirect → sh2.redirect
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Redirect()) or sh2Truthy(sh2Exec()));
    mv_result = sh2Capture();
    try stdout.print("{s}{s}\n", .{"Move result: ", mv_result});
    try stdout.flush(); // line 20
    // TODO(unsupported): redirect → sh2.redirect
    // TODO(unsupported): redirect → sh2.redirect
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Redirect()) or sh2Truthy(sh2Exec()));
    rm_result = sh2Capture();
    try stdout.print("{s}{s}\n", .{"Remove result: ", rm_result});
    try stdout.flush(); // line 26
    // TODO(unsupported): redirect → sh2.redirect
    // TODO(unsupported): redirect → sh2.redirect
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Redirect()) or sh2Truthy(sh2Exec()));
    mkdir_result = sh2Capture();
    try stdout.print("{s}{s}\n", .{"Mkdir result: ", mkdir_result});
    try stdout.flush(); // line 32
    _ = sh2Run(&[_][]const u8{"touch", "test_dir/file"}); // line 33
    // TODO(unsupported): redirect → sh2.redirect
    // TODO(unsupported): redirect → sh2.redirect
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Redirect()) or sh2Truthy(sh2Exec()));
    touch_result = sh2Capture();
    try stdout.print("{s}{s}\n", .{"Touch result: ", touch_result});
    try stdout.flush(); // line 39
    _ = sh2Run(&[_][]const u8{"rm", "-f", "test_file.txt", "test_file_copy.txt", "test_file_moved.txt"}); // line 42
    // TODO(unsupported): redirect → sh2.redirect // line 43
    // TODO(unsupported): redirect → sh2.redirect
    _ = (sh2Truthy(sh2Redirect()) or true);
    try stdout.print("{s}\n", .{"=== File Manipulation Commands Complete ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"cd", "/"}); // line 46
    _ = sh2Run(&[_][]const u8{"rm", "-r", "-f", d}); // line 47
}
// 22 construct(s) lowered to TODO markers