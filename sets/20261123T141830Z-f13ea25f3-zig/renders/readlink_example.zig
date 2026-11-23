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

    var target: []const u8 = "";
    var target2: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    target = sh2Capture(); // line 3
    try stdout.print("{s}{s}\n", .{"vi resolves to: ", target});
    try stdout.flush(); // line 4
    target2 = sh2Capture(); // line 5
    try stdout.print("{s}{s}\n", .{"python3 resolves to: ", target2});
    try stdout.flush(); // line 6
}