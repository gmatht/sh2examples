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

    var current_date: []const u8 = "";
    var current_dir: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    try stdout.print("{s}\n", .{"=== Basic Command Substitution ==="});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"Current date: ", sh2Capture()});
    try stdout.flush(); // line 9
    try stdout.print("{s}{s}\n", .{"Current directory: ", sh2Capture()});
    try stdout.flush(); // line 11
    current_date = sh2Capture(); // line 14
    current_dir = sh2Capture(); // line 16
    try stdout.print("{s}{s}\n", .{"Stored date: ", current_date});
    try stdout.flush(); // line 18
    try stdout.print("{s}{s}\n", .{"Stored directory: ", current_dir});
    try stdout.flush(); // line 20
    try stdout.print("{s}\n", .{"=== Basic Command Substitution Complete ==="});
    try stdout.flush();
}