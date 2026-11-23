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

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    _ = sh2Run(&[_][]const u8{"set", "-e", "-u", "-o", "pipefail"}); // line 6
    try stdout.print("{s}\n", .{"== Case modification in parameter expansion =="});
    try stdout.flush();
    try stdout.print("{s}\n", .{"WORLD"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"world"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"World"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"== Advanced parameter expansion =="});
    try stdout.flush();
    try stdout.print("{s}\n", .{"013_param_expansion_file.txt"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"/tmp"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"aXXa"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"== More parameter expansion =="});
    try stdout.flush();
    try stdout.print("{s}\n", .{" world"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"hello "});
    try stdout.flush();
    try stdout.print("{s}\n", .{"hell0 w0rld"});
    try stdout.flush();
    try stdout.print("{s}\n", .{"== Default values =="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"unset", "maybe"}); // line 27
    // TODO(unsupported): param → sh2.param // line 28
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 29
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 30
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{sh2Param()});
    try stdout.flush();
}
// 6 construct(s) lowered to TODO markers