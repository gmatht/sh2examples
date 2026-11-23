const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var x: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    // TODO(unsupported): param → sh2.param // line 2
    // TODO(unsupported): param → sh2.param
    x = std.fmt.allocPrint(std.heap.page_allocator, "{s}extra", .{sh2Param()}) catch "";
    try stdout.print("{s}\n", .{x});
    try stdout.flush(); // line 3
}
// 2 construct(s) lowered to TODO markers