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

fn sh2Pipeline() []const u8 {
    sh2TODO("pipeline");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var i: i64 = 0;
    var n: i64 = 0;
    var sum: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    // TODO(unsupported): pipeline → sh2.pipeline
    // TODO(unsupported): pipeline → sh2.pipeline
    _ = sh2Pipeline();
    { // line 17
        const sh2_items0 = [_]i64{4, 9, 16, 25, 100};
        for (sh2_items0[0..]) |sh2_v1| {
            n = sh2_v1;
            try stdout.print("{s}{d}{s}{s}\n", .{"sqrt(", n, ") = ", sh2Capture()});
    try stdout.flush();
        }
    }
    sum = "0"; // line 21
    { // line 22
        const sh2_items2 = [_]i64{1, 2, 3, 4, 5};
        for (sh2_items2[0..]) |sh2_v3| {
            i = sh2_v3;
            sum = sh2Capture();
        }
    }
    try stdout.print("{s}{s}\n", .{"sum=", sum});
    try stdout.flush(); // line 25
    _ = &i;
}
// 14 construct(s) lowered to TODO markers