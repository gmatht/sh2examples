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

    var existing: []const u8 = "";
    var full: []const u8 = "";
    var missing: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    existing = sh2Capture(); // line 3
    missing = sh2Capture(); // line 4
    full = sh2Capture(); // line 5
    try stdout.print("{s}{s}\n", .{"Existing: ", existing});
    try stdout.flush(); // line 6
    try stdout.print("{s}{s}\n", .{"Missing:  ", missing});
    try stdout.flush(); // line 7
    try stdout.print("{s}{s}\n", .{"Full:     ", full});
    try stdout.flush(); // line 8
}