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

fn sh2Join() []const u8 {
    sh2TODO("join");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var IFS: []const u8 = "";
    var config__host__: []const u8 = "";
    var config__port__: []const u8 = "";
    var config__user__: []const u8 = "";
    var sorted: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    _ = sh2Run(&[_][]const u8{"declare", "-A", "config"}); // line 4
    config__user__ = "admin";
    config__host__ = "localhost";
    config__port__ = "8080";
    IFS = "\n"; // line 10
    // TODO(unsupported): capture → sh2.capture
    // TODO(unsupported): capture → sh2.capture
    sorted = [_] []const u8{sh2Capture()};
    // TODO(unsupported): join → sh2.join // line 11
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}{s}\n", .{"Config: ", sh2Join()});
    try stdout.flush();
    _ = &IFS;
    _ = &config__host__;
    _ = &config__port__;
    _ = &config__user__;
    _ = &sorted;
}
// 4 construct(s) lowered to TODO markers