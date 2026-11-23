const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Join() []const u8 {
    sh2TODO("join");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var x: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    x = "hello world";
    // TODO(unsupported): join → sh2.join // line 4
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}\n", .{sh2Join()});
    try stdout.flush();
    // TODO(unsupported): join → sh2.join // line 5
    // TODO(unsupported): join → sh2.join
    try stdout.print("{s}\n", .{sh2Join()});
    try stdout.flush();
    _ = &x;
}
// 4 construct(s) lowered to TODO markers