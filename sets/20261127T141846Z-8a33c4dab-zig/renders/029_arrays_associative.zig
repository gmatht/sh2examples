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

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}

fn sh2Pipeline() []const u8 {
    sh2TODO("pipeline");
    return "";
}

pub fn main(init: std.process.Init) !void {

    const k: []const u8 = "";
    var map_answer_: []const u8 = "";
    var map_foo_: []const u8 = "";
    var map_two_: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    _ = sh2Run(&[_][]const u8{"set", "-e", "-u", "-o", "pipefail"}); // line 4
    try stdout.print("{s}\n", .{"== Associative arrays =="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"declare", "-A", "map"}); // line 7
    map_foo_ = "bar"; // line 8
    map_answer_ = "42"; // line 9
    map_two_ = "1 + 1";
    // TODO(unsupported): param → sh2.param // line 11
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 12
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{sh2Param()});
    try stdout.flush();
    // TODO(unsupported): pipeline → sh2.pipeline // line 15
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    _ = &k;
    _ = &map_answer_;
    _ = &map_foo_;
    _ = &map_two_;
}
// 6 construct(s) lowered to TODO markers