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

fn sh2Param() []const u8 {
    sh2TODO("param");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var iface: []const u8 = "";
    var ip: []const u8 = "";
    var pretty_name: []const u8 = "";

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    pretty_name = sh2Capture();
    // TODO(unsupported): capture → sh2.capture
    // TODO(unsupported): capture → sh2.capture
    {
        const sh2_items0 = [_][]const u8{sh2Capture()};
        for (sh2_items0[0..]) |sh2_v1| {
            iface = sh2_v1;
            if (!std.mem.eql(u8, iface, "")) {
                ip = sh2Capture();
                try stdout.print("{s}{s}{s}", .{"pretty_name=[", pretty_name, "]\n"});
    try stdout.flush();
                // TODO(unsupported): param → sh2.param
                // TODO(unsupported): param → sh2.param
                try stdout.print("{s}\n", .{sh2Param()});
    try stdout.flush();
            }
        }
    }
    _ = &ip;
}
// 4 construct(s) lowered to TODO markers