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

    var formatted_date: []const u8 = "";
    var yes_result: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    try stdout.print("{s}\n", .{"=== System Utilities ==="});
    try stdout.flush();
    formatted_date = sh2Capture(); // line 11
    try stdout.print("{s}{s}\n", .{"Formatted date: ", formatted_date});
    try stdout.flush(); // line 13
    yes_result = sh2Capture();
    try stdout.print("{s}\n", .{"Yes command result:"});
    try stdout.flush();
    try stdout.print("{s}\n", .{yes_result});
    try stdout.flush(); // line 35
}