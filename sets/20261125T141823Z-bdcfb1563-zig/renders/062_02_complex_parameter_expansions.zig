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

    var complex_var: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    try stdout.print("{s}\n", .{"Testing complex parameter expansions..."});
    try stdout.flush();
    complex_var = "hello world";
    // TODO(unsupported): param → sh2.param // line 6
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 7
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 8
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 9
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}\n", .{sh2Param()});
    try stdout.flush();
    _ = &complex_var;
}
// 8 construct(s) lowered to TODO markers