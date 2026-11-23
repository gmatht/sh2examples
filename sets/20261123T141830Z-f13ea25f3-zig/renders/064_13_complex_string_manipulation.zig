const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var basename: []const u8 = "";
    var extension: []const u8 = "";
    var filename: []const u8 = "";
    const lowercase: []const u8 = "";
    const uppercase: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    filename = "my_file.txt";
    // TODO(unsupported): param → sh2.param // line 5
    // TODO(unsupported): param → sh2.param
    basename = sh2Param();
    // TODO(unsupported): param → sh2.param // line 6
    // TODO(unsupported): param → sh2.param
    extension = sh2Param();
    try stdout.print("{s}{s}\n", .{"Basename: ", basename});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"Extension: ", extension});
    try stdout.flush();
    try stdout.print("{s}{s}\n", .{"Uppercase: ", "MY_FILE.TXT"});
    try stdout.flush(); // line 10
    try stdout.print("{s}{s}\n", .{"Lowercase: ", "my_file.txt"});
    try stdout.flush(); // line 11
    _ = &filename;
    _ = &lowercase;
    _ = &uppercase;
}
// 4 construct(s) lowered to TODO markers