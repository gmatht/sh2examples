const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2ToInt(s: []const u8) i64 {
    return std.fmt.parseInt(i64, std.mem.trim(u8, s, " \t\n\r"), 10) catch 0;
}

fn sh2Capture() []const u8 {
    sh2TODO("capture");
    return "";
}

fn sh2GetVar() []const u8 {
    sh2TODO("getVar");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var LIBSSL: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    LIBSSL = sh2Capture(); // line 4
    if ((sh2ToInt(sh2GetVar()) == 2)) {
        try stdout.print("{s}{s}{s}", .{"LIBSSL=[", LIBSSL, "]\n"});
    try stdout.flush();
        try stdout.print("{s}\n", .{"error"});
    try stdout.flush();
    }
}