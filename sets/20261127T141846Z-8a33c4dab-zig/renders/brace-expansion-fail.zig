const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2ArrayItems() []const u8 {
    sh2TODO("arrayItems");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    try stdout.print("{s}\n", .{"1 2 3 4 5"});
    try stdout.flush();
    // TODO(unsupported): arrayItems → sh2.arrayItems // line 5
    // TODO(unsupported): arrayItems → sh2.arrayItems
    try stdout.print("{s}\n", .{sh2ArrayItems()});
    try stdout.flush();
}
// 2 construct(s) lowered to TODO markers