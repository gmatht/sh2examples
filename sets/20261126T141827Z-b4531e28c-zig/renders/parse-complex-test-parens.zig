const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

fn sh2TestStub() bool {
    sh2TODO("test");
    return false;
}

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    if ((sh2Test() and sh2Truthy(sh2Exec()))) {
        try stdout.print("{s}\n", .{"complex condition met"});
    try stdout.flush();
    }
}
// 2 construct(s) lowered to TODO markers