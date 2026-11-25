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

    var s: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    s = "abc:def:ghi"; // line 3
    // TODO(unsupported): param → sh2.param // line 4
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"short-prefix=", sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 5
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"long-prefix=", sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 6
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"short-suffix=", sh2Param()});
    try stdout.flush();
    // TODO(unsupported): param → sh2.param // line 7
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}\n", .{"long-suffix=", sh2Param()});
    try stdout.flush();
    _ = &s;
}
// 8 construct(s) lowered to TODO markers