const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2B2S(b: bool) []const u8 {
    return if (b) "1" else "0";
}

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

fn sh2TestStub() bool {
    sh2TODO("test");
    return false;
}

pub fn main(init: std.process.Init) !void {

    var __sh2_tc0: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    __sh2_tc0 = sh2Capture(); // line 3
    _ = sh2B2S(sh2Test());
    try stdout.print("{s}", .{"parsed OK\n"});
    try stdout.flush();
    _ = &__sh2_tc0;
}