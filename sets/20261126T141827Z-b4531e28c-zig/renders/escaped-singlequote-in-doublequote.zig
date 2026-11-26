const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var conffile: []const u8 = "";
    var pkg: []const u8 = "";
    var x: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    pkg = "dummy"; // line 3
    conffile = "test"; // line 4
    x = sh2Capture(); // line 5
    try stdout.print("{s}{s}{s}", .{"result=[", x, "]\n"});
    try stdout.flush(); // line 7
    try stdout.print("{s}", .{"pkg=[dummy]\n"});
    try stdout.flush();
    try stdout.print("{s}", .{"conffile=[test]\n"});
    try stdout.flush();
    _ = &conffile;
    _ = &pkg;
}