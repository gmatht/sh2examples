const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

var sh2_io: ?std.Io = null;

fn sh2Run(argv: []const []const u8) []const u8 {
    const io = sh2_io orelse return "";
    const result = std.process.run(std.heap.page_allocator, io, .{
        .argv = argv,
        .stdout_limit = .limited(1 << 20),
    }) catch return "";
    return std.mem.trimEnd(u8, result.stdout, "\n");
}

fn sh2Pipeline() []const u8 {
    sh2TODO("pipeline");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var SHELL_VAR: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;
    sh2_io = init.io;

    try stdout.print("{s}\n", .{"Warmup 1"});
    try stdout.flush(); // line 3
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    try stdout.print("{s}\n", .{"=== Example 1: Simple Perl one-liner ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"perl", "-e", "print \"Hello from Perl!\\n\""}); // line 12
    try stdout.print("{s}\n", .{"\n=== Example 2: Perl with arguments ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"perl", "-e", "foreach $arg (@ARGV) { print \"Argument: $arg\\n\" }", "first", "second", "third"});
    try stdout.print("{s}\n", .{"\n=== Example 3: Perl processing shell variables ==="});
    try stdout.flush();
    SHELL_VAR = "Hello World";
    _ = sh2Run(&[_][]const u8{"perl", "-e", "print \"Shell variable: $ENV{SHELL_VAR}\\n\""}); // line 21
    _ = sh2Run(&[_][]const u8{"export", "SHELL_VAR"}); // line 22
    _ = sh2Run(&[_][]const u8{"perl", "-e", "print \"Shell variable: $ENV{SHELL_VAR}\\n\""}); // line 23
    try stdout.print("{s}\n", .{"\n=== Example 4: Perl reading from pipeline ==="});
    try stdout.flush();
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    try stdout.print("{s}\n", .{"\n=== Example 5: Perl script with here document ==="});
    try stdout.flush();
    _ = sh2Run(&[_][]const u8{"perl"}); // line 31
    _ = &SHELL_VAR;
}
// 4 construct(s) lowered to TODO markers