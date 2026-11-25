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

    var n: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    n = "5"; // line 4
    if ((n == 5)) {
        try stdout.print("{s}\n", .{"equal"});
    try stdout.flush();
    }
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}{s}{s}", .{"n", "=[", sh2Param(), "]\n"});
    try stdout.flush();
}
// 2 construct(s) lowered to TODO markers