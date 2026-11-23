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

    var __ps_tmp2: []const u8 = "";
    var __ps_tmp3: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    _ = sh2Run(&[_][]const u8{"set", "-e", "-u", "-o", "pipefail"}); // line 4
    try stdout.print("{s}\n", .{"== More process substitution examples =="});
    try stdout.flush();
    // TODO(unsupported): redirect → sh2.redirect
    // TODO(unsupported): redirect → sh2.redirect
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Redirect()) or sh2Truthy(sh2Exec()));
    __ps_tmp2 = sh2Capture();
    try std.fs.cwd().writeFile(.{ .sub_path = __ps_tmp2, .data = "name1\nname2\n" });
    __ps_tmp3 = sh2Capture();
    try std.fs.cwd().writeFile(.{ .sub_path = __ps_tmp3, .data = "value1\nvalue2\n" });
    _ = sh2Run(&[_][]const u8{"paste", __ps_tmp2, __ps_tmp3});
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = sh2Exec();
    try stdout.print("{s}{s}\n", .{"status: ", "ok"});
    try stdout.flush();
}
// 6 construct(s) lowered to TODO markers