const std = @import("std");

// sh2.* runtime helpers — TODO: implement (harness/sh2-namespace.json)
fn sh2TODO(name: []const u8) noreturn {
    std.debug.print("TODO sh2.{s}\n", .{name});
    std.process.exit(2);
}

fn sh2Truthy(s: []const u8) bool {
    return s.len != 0;
}

fn sh2Continue() []const u8 {
    sh2TODO("continue");
    return "";
}

pub fn main(init: std.process.Init) !void {

    var i: i64 = 0;

    var stdout_buffer: [4096]u8 = undefined;
    var stdout_file_writer: std.Io.File.Writer = .init(.stdout(), init.io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    {
        const sh2_items0 = [_]i64{1, 2, 3};
        for (sh2_items0[0..]) |sh2_v1| {
            i = sh2_v1;
            // TODO(unsupported): continue → sh2.continue
            // TODO(unsupported): continue → sh2.continue
            _ = ((!(i != 2)) or sh2Truthy(sh2Continue()));
            try stdout.print("{d}\n", .{i});
    try stdout.flush();
        }
    }
}
// 2 construct(s) lowered to TODO markers