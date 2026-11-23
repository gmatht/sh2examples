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

fn sh2Exec() []const u8 {
    sh2TODO("exec");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var INTERPRETER_UNDER_TEST: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    INTERPRETER_UNDER_TEST = sh2Capture(); // line 4
    if ((!!std.mem.eql(u8, INTERPRETER_UNDER_TEST, ""))) {
        try stdout.print("{s}\n", .{"Interpreter must be the command line argument."});
    try stdout.flush();
        std.process.exit(@intCast(4));
    }
    // TODO(unsupported): exec → sh2.exec // line 9
    // TODO(unsupported): exec → sh2.exec
    _ = sh2Exec();
}
// 2 construct(s) lowered to TODO markers