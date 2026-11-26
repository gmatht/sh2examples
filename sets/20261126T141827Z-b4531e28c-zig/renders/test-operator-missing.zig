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

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    if (sh2Test()) { // line 3
    }
    if (sh2Test()) { // line 4
    }
    if (sh2Test()) { // line 5
    }
    if (sh2Test()) { // line 6
    }
    if (sh2Test()) { // line 7
    }
    if (sh2Test()) { // line 8
    }
    if (sh2Test()) { // line 9
    }
    if (sh2Test()) { // line 10
    }
    if (sh2Test()) { // line 11
    }
    if (sh2Test()) { // line 12
    }
    if (sh2Test()) { // line 14
    }
    if (sh2Test()) { // line 15
    }
    if (sh2Test()) { // line 16
    }
    if (true) {
    }
    if (true) {
    }
    try stdout.print("{s}", .{"parsed OK\n"});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"status: ", "ok"});
    try stdout.flush();
}