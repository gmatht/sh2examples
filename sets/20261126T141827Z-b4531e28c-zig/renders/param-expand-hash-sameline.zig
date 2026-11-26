const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2TestStub() bool {
    sh2TODO("test");
    return false;
}

pub fn main(init: std.process.Init) !void {

    var MAXWAIT: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    MAXWAIT = "20 10";
    if (sh2Test()) {
        try stdout.print("{s}\n", .{"same line"});
    try stdout.flush();
    }
    _ = &MAXWAIT;
}