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

    var f: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    // TODO(unsupported): capture → sh2.capture // line 6
    // TODO(unsupported): capture → sh2.capture
    {
        const sh2_items0 = [_][]const u8{sh2Capture()};
        for (sh2_items0[0..]) |sh2_v1| {
            f = sh2_v1;
            try stdout.print("{s}\n", .{f});
    try stdout.flush();
        }
    }
}
// 2 construct(s) lowered to TODO markers