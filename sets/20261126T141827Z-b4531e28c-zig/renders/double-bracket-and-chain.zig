const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

fn sh2Redirect() []const u8 {
    sh2TODO("redirect");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var __sh2_tc0: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    __sh2_tc0 = sh2Capture();
    // TODO(unsupported): redirect → sh2.redirect
    // TODO(unsupported): redirect → sh2.redirect
    if (((__sh2_tc0 .len != 0) and sh2Truthy(sh2Redirect()))) {
        try stdout.print("{s}\n", .{"found"});
    try stdout.flush();
    }
}
// 2 construct(s) lowered to TODO markers