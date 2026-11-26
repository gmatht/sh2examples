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

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}

fn sh2Redirect() []const u8 {
    sh2TODO("redirect");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    // TODO(unsupported): redirect → sh2.redirect // line 3
    // TODO(unsupported): redirect → sh2.redirect
    // TODO(unsupported): exec → sh2.exec
    // TODO(unsupported): exec → sh2.exec
    _ = (sh2Truthy(sh2Redirect()) and sh2Truthy(sh2Exec()));
    // TODO(unsupported): param → sh2.param
    // TODO(unsupported): param → sh2.param
    try stdout.print("{s}{s}{s}{s}", .{"n", "=[", sh2Param(), "]\n"});
    try stdout.flush();
}
// 6 construct(s) lowered to TODO markers